use {
    crate::*,
    crossterm::style::Color::*,
    lfs_core::*,
    termimad::{
        minimad::{*, Alignment::*},
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
            if args.inodes && stats.files > 0 {
                let iuse_share = stats.inodes_use_share();
                let ipb = ProgressBar::new(iuse_share as f32, INODES_BAR_WIDTH);
                sub
                    .set("inodes", stats.files)
                    .set("iused", stats.inodes_used())
                    .set("iuse-percents", format!("{:.0}%", 100.0 * iuse_share))
                    .set("ibar", format!("{:<width$}", ipb, width = INODES_BAR_WIDTH))
                    .set("ifree", stats.favail);
            }
        }
    }
    let skin = if color {
        make_colored_skin()
    } else {
        MadSkin::no_style()
    };

    let mut tbl = TableBuilder::default();
    tbl
        .col(Col::simple("id").align(Right))
        .col(Col::new("dev", "${dev-major}:${dev-minor}"))
        .col(Col::simple("filesystem"));
    if args.labels {
        tbl.col(Col::simple("label"));
    }
    tbl
        .col(Col::simple("disk").align_content(Center))
        .col(Col::simple("type"));
    tbl
        .col(Col::new("used", "~~${used}~~"))
        .col(Col::new("use%", "~~${use-percents}~~ `${bar}`").align_content(Right))
        .col(Col::new("free", "*${free}*").align(Right))
        .col(Col::new("size", "**${size}**"));
    if args.inodes {
        tbl
            //.col(Col::new("used inodes", "~~${iused}~~").align_content(Right))
            //.col(Col::new("free inodes", "*${ifree}*").align(Right))
            .col(Col::new("inodes use", "~~${iuse-percents}~~ `${ibar}`").align_content(Right));
            //.col(Col::new("inodes", "**${inodes}**").align_content(Right));
    }
    tbl
        .col(Col::simple("mount point").align(Left));


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

