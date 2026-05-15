use {
    crate::{
        Args,
        col::Col,
    },
    lfs_core::*,
    std::{
        borrow::Cow,
        io::Write,
    },
    termimad::{
        CompoundStyle,
        MadSkin,
        ProgressBar,
        StrFit,
        crossterm::style::Color::*,
        minimad::{
            self,
            OwningTemplateExpander,
            TableBuilder,
        },
    },
};

// those colors are chosen to be "redish" for used, "greenish" for available
// and, most importantly, to work on both white and black backgrounds. If you
// find a better combination, please show me.
static USED_COLOR: u8 = 209;
static AVAI_COLOR: u8 = 65;
static SIZE_COLOR: u8 = 172;
static BAR_WIDTH_SPACE_THRESHOLD: usize = 4;

pub fn write<W: Write>(
    w: &mut W,
    mounts: &[&Mount],
    color: bool,
    args: &Args,
) -> std::io::Result<()> {
    if args.cols.is_empty() {
        return Ok(());
    }
    let units = args.units;
    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");
    let use_col_width = compute_use_col_width(args);
    for mount in mounts {
        let sub = expander
            .sub("rows")
            .set(
                "id",
                mount
                    .info
                    .id
                    .as_ref()
                    .map_or("".to_string(), |i| i.to_string()),
            )
            .set("dev", mount.info.dev)
            .set("filesystem", &mount.info.fs)
            .set("disk", mount.disk.as_ref().map_or("", |d| d.disk_type()))
            .set("type", &mount.info.fs_type)
            .set("mount-point", mount.info.mount_point.to_string_lossy())
            .set("mount-options", mount.info.options_string())
            .set_option("uuid", mount.uuid.as_ref())
            .set_option("part_uuid", mount.part_uuid.as_ref())
            .set_option("compress-level", mount.info.option_value("compress"));
        if let Some(label) = &mount.fs_label {
            sub.set("label", label);
        }
        if mount.is_remote() {
            sub.set("remote", "x");
        }
        if let Some(stats) = mount.stats() {
            let use_share = stats.use_share();
            let free_share = 1.0 - use_share;
            if args.bar_width > BAR_WIDTH_SPACE_THRESHOLD {
                sub.set("use-space", " ");
            } else {
                // if the bar has been set to a very small width, the user probably don't
                // want space to be wasted
                sub.set("use-space", "");
            }
            sub.set("size", units.fmt(stats.size()))
                .set("used", units.fmt(stats.used()))
                .set("use-percents", format!("{:.0}%", 100.0 * use_share))
                .set_md(
                    "bar",
                    progress_bar_md(use_share, args.bar_width, args.ascii),
                )
                .set("free", units.fmt(stats.available()))
                .set("free-percents", format!("{:.0}%", 100.0 * free_share));
            if let Some(inodes) = &stats.inodes {
                let iuse_share = inodes.use_share();
                sub.set("inodes", inodes.files)
                    .set("iused", inodes.used())
                    .set("iuse-percents", format!("{:.0}%", 100.0 * iuse_share))
                    .set_md(
                        "ibar",
                        progress_bar_md(iuse_share, args.bar_width, args.ascii),
                    )
                    .set("ifree", inodes.favail);
            }
        } else if mount.is_timeout() {
            sub.set("use-error", string_fitting_cols("timeout", use_col_width));
        } else if mount.is_unreachable() {
            sub.set(
                "use-error",
                string_fitting_cols("unreachable", use_col_width),
            );
        }
    }
    let mut skin = if color {
        make_colored_skin()
    } else {
        MadSkin::no_style()
    };
    if args.ascii {
        skin.limit_to_ascii();
    }

    let mut tbl = TableBuilder::default();
    for col in args.cols.cols() {
        tbl.col(
            minimad::Col::new(
                col.title(),
                match col {
                    Col::Id => "${id}",
                    Col::Dev => "${dev}",
                    Col::Filesystem => "${filesystem}",
                    Col::Label => "${label}",
                    Col::Disk => "${disk}",
                    Col::Type => "${type}",
                    Col::Remote => "${remote}",
                    Col::Used => "~~${used}~~",
                    Col::Use => "~~${use-percents}~~${use-space}${bar}~~${use-error}~~",
                    Col::UsePercent => "~~${use-percents}~~",
                    Col::Free => "*${free}*",
                    Col::FreePercent => "*${free-percents}*",
                    Col::Size => "**${size}**",
                    Col::InodesFree => "*${ifree}*",
                    Col::InodesUsed => "~~${iused}~~",
                    Col::InodesUse => "~~${iuse-percents}~~ ${ibar}",
                    Col::InodesUsePercent => "~~${iuse-percents}~~",
                    Col::InodesCount => "**${inodes}**",
                    Col::MountPoint => "${mount-point}",
                    Col::Uuid => "${uuid}",
                    Col::PartUuid => "${part_uuid}",
                    Col::MountOptions => "${mount-options}",
                    Col::CompressLevel => "${compress-level}",
                },
            )
            .align_content(col.content_align())
            .align_header(col.header_align()),
        );
    }

    skin.write_owning_expander_md(w, &expander, &tbl)
}

/// Use settings and heuristics to determine the max width to be used by errors messages
/// in the "use" column, so that they don't break the layout too much.
fn compute_use_col_width(args: &Args) -> usize {
    const MAX: usize = 20; // basically no limit
    let mut width = 3 + args.bar_width; // 3 for eg "97%"
    if args.bar_width > BAR_WIDTH_SPACE_THRESHOLD {
        width += 1;
    }
    if width < MAX {
        if args.cols.len() < 3 {
            return MAX;
        }
        let (terminal_width, _) = termimad::terminal_size();
        if terminal_width > 150 {
            return MAX;
        }
    }
    width
}

/// Return a string potentially shortened with an ellipsis, so that it fits in the given number of
/// columns. To avoid importing a crate like unicode_width, all characters are assumed to have a
/// width of 1. To ease enforcing this assumption, only static string are used.
fn string_fitting_cols(
    s: &'static str,
    cols: usize,
) -> Cow<'static, str> {
    if cols == 0 || s.chars().count() <= cols {
        Cow::Borrowed(s)
    } else {
        let shortened = StrFit::make_cow(s, cols - 1).0;
        format!("{}…", shortened).into()
    }
}

fn make_colored_skin() -> MadSkin {
    MadSkin {
        bold: CompoundStyle::with_fg(AnsiValue(SIZE_COLOR)), // size
        inline_code: CompoundStyle::with_fgbg(AnsiValue(USED_COLOR), AnsiValue(AVAI_COLOR)), // use bar
        strikeout: CompoundStyle::with_fg(AnsiValue(USED_COLOR)),                            // use%
        italic: CompoundStyle::with_fg(AnsiValue(AVAI_COLOR)), // available
        ..Default::default()
    }
}

fn progress_bar_md(
    share: f64,
    bar_width: usize,
    ascii: bool,
) -> String {
    if ascii {
        let count = (share * bar_width as f64).round() as usize;
        let bar: String = "".repeat(count);
        let no_bar: String = "-".repeat(bar_width - count);
        format!("~~{}~~*{}*", bar, no_bar)
    } else {
        let pb = ProgressBar::new(share as f32, bar_width);
        format!("`{:<width$}`", pb, width = bar_width)
    }
}
