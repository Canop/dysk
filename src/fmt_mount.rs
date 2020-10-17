use {
    lfs_core::*,
    crossterm::style::Color::*,
    file_size,
    minimad::{
        TextTemplate,
        OwningTemplateExpander,
    },
    termimad::{
        CompoundStyle,
        FmtText,
        MadSkin,
        terminal_size,
    },
};

static MD: &str = r#"
|-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:
|id|dev|filesystem|dsk|type|size|used|use%|avail|mount point
|-:|:-|:-|:-:|:-:|-:|-:|-:|-:|:-
${mount-points
|${id}|${dev-major}:${dev-minor}|${fs}|${dsk}|${fs-type}|*${size}*|`${used}`|`${use-percents}`|**${available}**|${mount-point}
}
|-:
"#;

pub fn print(mounts: &[Mount]) -> Result<()> {
    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");
    for mount in mounts {
        let sub = expander.sub("mount-points")
            .set("id", format!("{}", mount.info.id))
            .set("dev-major", format!("{}", mount.info.dev.major))
            .set("dev-minor", format!("{}", mount.info.dev.minor))
            .set("fs", &mount.info.fs)
            .set("dsk", mount.disk.as_ref().map_or("", |d| d.disk_type()))
            .set("fs-type", &mount.info.fs_type)
            .set("mount-point", mount.info.mount_point.to_string_lossy());
        if let Some(stats) = mount.stats.as_ref().filter(|s| s.size() > 0) {
            sub
                .set("size", file_size::fit_4(stats.size()))
                .set("used", file_size::fit_4(stats.used()))
                .set("use-percents", format!("{:.0}%", 100.0*stats.use_share()))
                .set("available", file_size::fit_4(stats.available()));
        }
    }
    let (width, _) = terminal_size();
    let template = TextTemplate::from(MD);
    let text = expander.expand(&template);
    let mut skin = MadSkin::default();
    skin.italic = CompoundStyle::with_fg(AnsiValue(209));
    skin.inline_code = CompoundStyle::with_fg(AnsiValue(166));
    skin.bold = CompoundStyle::with_fg(AnsiValue(208));
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    print!("{}", fmt_text);
    Ok(())
}

