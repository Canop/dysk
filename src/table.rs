use {
    crate::{
        Args, col::Col,
    },
    crossterm::style::Color::*,
    lfs_core::*,
    termimad::{
        minimad::{self, OwningTemplateExpander, TableBuilder},
        CompoundStyle, MadSkin, ProgressBar,
    },
};

// those colors are chosen to be "redish" for used, "greenish" for available
// and, most importantly, to work on both white and black backgrounds. If you
// find a better combination, please show me.
static USED_COLOR: u8 = 209;
static AVAI_COLOR: u8 = 65;
static SIZE_COLOR: u8 = 172;

static BAR_WIDTH: usize = 5;
static INODES_BAR_WIDTH: usize = 5;

pub fn print(mounts: &[Mount], color: bool, args: &Args) {
    if args.cols.is_empty() {
        return;
    }
    let units = args.units;
    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");
    for mount in mounts {
        let sub = expander
            .sub("rows")
            .set("id", mount.info.id)
            .set("dev-major", mount.info.dev.major)
            .set("dev-minor", mount.info.dev.minor)
            .set("filesystem", &mount.info.fs)
            .set("disk", mount.disk.as_ref().map_or("", |d| d.disk_type()))
            .set("type", &mount.info.fs_type)
            .set("mount-point", mount.info.mount_point.to_string_lossy());
        if let Some(label) = &mount.fs_label {
            sub.set("label", label);
        }
        if let Some(stats) = mount.stats.as_ref().filter(|s| s.size() > 0) {
            let use_share = stats.use_share();
            let pb = ProgressBar::new(use_share as f32, BAR_WIDTH);
            sub
                .set("size", units.fmt(stats.size()))
                .set("used", units.fmt(stats.used()))
                .set("use-percents", format!("{:.0}%", 100.0 * use_share))
                .set("bar", format!("{:<width$}", pb, width = BAR_WIDTH))
                .set("free", units.fmt(stats.available()));
            if let Some(inodes) = &stats.inodes {
                let iuse_share = inodes.use_share();
                let ipb = ProgressBar::new(iuse_share as f32, INODES_BAR_WIDTH);
                sub
                    .set("inodes", inodes.files)
                    .set("iused", inodes.used())
                    .set("iuse-percents", format!("{:.0}%", 100.0 * iuse_share))
                    .set("ibar", format!("{:<width$}", ipb, width = INODES_BAR_WIDTH))
                    .set("ifree", inodes.favail);
            }
        }
    }
    let skin = if color {
        make_colored_skin()
    } else {
        MadSkin::no_style()
    };

    let mut tbl = TableBuilder::default();
    for col in args.cols.cols() {
        tbl.col(
            minimad::Col::new(
                col.title(),
                match col {
                    Col::Id => "${id}",
                    Col::Dev => "${dev-major}:${dev-minor}",
                    Col::Filesystem => "${filesystem}",
                    Col::Label => "${label}",
                    Col::Disk => "${disk}",
                    Col::Type => "${type}",
                    Col::Used => "~~${used}~~",
                    Col::Use => "~~${use-percents}~~ `${bar}`",
                    Col::Free => "*${free}*",
                    Col::Size => "**${size}**",
                    Col::InodesFree => "*${ifree}*",
                    Col::InodesUsed => "~~${iused}~~",
                    Col::InodesUse => "~~${iuse-percents}~~ `${ibar}`",
                    Col::InodesCount => "**${inodes}**",
                    Col::MountPoint => "${mount-point}",
                }
            )
            .align_content(col.content_align())
            .align_header(col.header_align())
        );
    }

    skin.print_owning_expander_md(&expander, &tbl);
}

fn make_colored_skin() -> MadSkin {
    MadSkin {
        bold: CompoundStyle::with_fg(AnsiValue(SIZE_COLOR)), // size
        inline_code: CompoundStyle::with_fgbg(AnsiValue(USED_COLOR), AnsiValue(AVAI_COLOR)), // use bar
        strikeout: CompoundStyle::with_fg(AnsiValue(USED_COLOR)), // use%
        italic: CompoundStyle::with_fg(AnsiValue(AVAI_COLOR)), // available
        ..Default::default()
    }
}

