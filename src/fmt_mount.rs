use {
    crossterm::style::Color::*,
    file_size,
    lfs_core::*,
    minimad::{OwningTemplateExpander, TextTemplate},
    termimad::{terminal_size, CompoundStyle, FmtText, MadSkin, ProgressBar},
};

// those colors are chosen to be "redish" for used, "greenish" for available
// and, most importantly, to work on both white and black backgrrounds. If you
// find a better combination, please show me.
static USED_COLOR: u8 = 209;
static AVAI_COLOR: u8 = 65;
static SIZE_COLOR: u8 = 172;

static BAR_WIDTH: usize = 5;

static MD: &str = r#"
|-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:
|id|dev|filesystem|disk|type|used|use%|avail|size|mount point
|-:|:-|:-|:-:|:-:|-:|-:|-:|:-
${mount-points
|${id}|${dev-major}:${dev-minor}|${fs}|${disk}|${fs-type}|`${used}`|`${use-percents}` ~~${bar}~~|*${available}*|**${size}**|${mount-point}
}
|-:
"#;

pub fn print(mounts: &[Mount], color: bool) -> Result<()> {
    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");
    for mount in mounts {
        let sub = expander
            .sub("mount-points")
            .set("id", format!("{}", mount.info.id))
            .set("dev-major", format!("{}", mount.info.dev.major))
            .set("dev-minor", format!("{}", mount.info.dev.minor))
            .set("fs", &mount.info.fs)
            .set("disk", mount.disk.as_ref().map_or("", |d| d.disk_type()))
            .set("fs-type", &mount.info.fs_type)
            .set("mount-point", mount.info.mount_point.to_string_lossy());
        if let Some(stats) = mount.stats.as_ref().filter(|s| s.size() > 0) {
            let use_share = stats.use_share();
            let pb = ProgressBar::new(use_share as f32, BAR_WIDTH);
            sub.set("size", file_size::fit_4(stats.size()))
                .set("used", file_size::fit_4(stats.used()))
                .set("use-percents", format!("{:.0}%", 100.0 * use_share))
                .set("bar", format!("{:<width$}", pb, width = BAR_WIDTH))
                .set("available", file_size::fit_4(stats.available()));
        }
    }
    let (width, _) = terminal_size();
    let template = TextTemplate::from(MD);
    let text = expander.expand(&template);
    let skin = if color {
        make_colored_skin()
    } else {
        MadSkin::no_style()
    };
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    print!("{}", fmt_text);
    Ok(())
}

fn make_colored_skin() -> MadSkin {
    MadSkin {
        bold: CompoundStyle::with_fg(AnsiValue(SIZE_COLOR)), // size
        inline_code: CompoundStyle::with_fg(AnsiValue(USED_COLOR)), // use%
        strikeout: CompoundStyle::with_fgbg(AnsiValue(USED_COLOR), AnsiValue(AVAI_COLOR)), // use bar
        italic: CompoundStyle::with_fg(AnsiValue(AVAI_COLOR)), // available
        ..Default::default()
    }
}
