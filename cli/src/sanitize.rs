use std::borrow::Cow;

/// Replace C0 (including ESC), DEL, and C1 controls with `?`.
///
/// Mount metadata is read by lfs-core and may contain bytes chosen by an
/// unprivileged user (a FUSE mount point or device string, for example).
/// Table and CSV rendering must not write those bytes to a terminal.
/// JSON is left untouched: serde already escapes controls.
pub fn sanitize(s: &str) -> Cow<'_, str> {
    if !s.contains(char::is_control) {
        return Cow::Borrowed(s);
    }
    Cow::Owned(
        s.chars()
            .map(|c| if c.is_control() { '?' } else { c })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            args::Args,
            csv,
            json,
            table,
            units::Units,
        },
        clap::Parser,
        lfs_core::*,
        std::io::Cursor,
    };

    #[test]
    fn keeps_text_without_controls() {
        assert_eq!(sanitize("/dev/sda1"), "/dev/sda1");
        assert_eq!(sanitize("café /mnt"), "café /mnt");
    }

    #[test]
    fn replaces_c0_del_c1_and_esc() {
        // ESC, BEL, DEL, and a C1 CSI.
        assert_eq!(sanitize("a\u{1b}\u{7}\u{7f}\u{9b}b"), "a????b");
        assert!(!sanitize("a\u{1b}b").contains(char::is_control));
    }

    #[test]
    fn table_and_csv_are_neutralized_json_is_not() {
        let evil = mount("fuse\u{1b}[31msrc", "/mnt/\u{1b}]0;pwned\u{7}", true);
        let clean = mount("/dev/sda1", "/safe/path", false);
        let mounts = [&evil, &clean];

        // One column at a time so termimad's terminal-width wrapping cannot
        // split the sanitized text across lines.
        assert_col("fs", &mounts, &["fuse?[31msrc", "/dev/sda1"]);
        assert_col("mp", &mounts, &["/mnt/?]0;pwned?", "/safe/path"]);
        assert_col("type", &mounts, &["fu?se", "ext4"]);
        assert_col("label", &mounts, &["lab?el"]);
        assert_col("options", &mounts, &["rw,compress=zstd?:3"]);
        assert_col("uuid", &mounts, &["uu?id"]);
        assert_col("partuuid", &mounts, &["part?"]);
        assert_col("compress", &mounts, &["zstd?:3"]);

        let colored = render_table("mp", &mounts, true);
        assert!(
            !colored.contains("\u{1b}]0;pwned"),
            "OSC payload survived color output: {colored:?}"
        );
        assert!(!colored.contains('\u{7}'), "{colored:?}");
        assert!(colored.contains("/mnt/?]0;pwned?"), "{colored:?}");

        let args = args_with("fs+type+label+mp+options+uuid+partuuid+compress");
        let mut csv_buf = Cursor::new(Vec::new());
        csv::write(&mut csv_buf, &mounts, &args).unwrap();
        let csv_out = String::from_utf8(csv_buf.into_inner()).unwrap();
        assert_terminal_safe(&csv_out);
        for needle in [
            "/mnt/?]0;pwned?",
            "fuse?[31msrc",
            "fu?se",
            "lab?el",
            "rw,compress=zstd?:3",
            "uu?id",
            "part?",
            "zstd?:3",
            "/safe/path",
            "/dev/sda1",
        ] {
            assert!(csv_out.contains(needle), "csv missing {needle:?}: {csv_out:?}");
        }

        let json_out = serde_json::to_string(&json::output_value(&mounts, Units::Si)).unwrap();
        // serde escapes C0 (so the ESC byte is gone) and keeps the original text.
        assert!(json_out.contains(r"\u001b"), "{json_out}");
        assert!(json_out.contains("]0;pwned"), "{json_out}");
        assert!(json_out.contains("/safe/path"), "{json_out}");
        assert!(json_out.contains("lab\u{9d}el"), "{json_out}");
        assert!(!json_out.contains("/mnt/?]0;pwned?"), "{json_out}");
        assert!(!json_out.contains('\u{1b}'), "{json_out}");
    }

    fn assert_col(
        col: &str,
        mounts: &[&Mount],
        needles: &[&str],
    ) {
        let out = render_table(col, mounts, false);
        assert_terminal_safe(&out);
        for needle in needles {
            assert!(
                out.contains(needle),
                "{col} missing {needle:?} in {out:?}"
            );
        }
    }

    fn render_table(
        cols: &str,
        mounts: &[&Mount],
        color: bool,
    ) -> String {
        let mut buf = Cursor::new(Vec::new());
        table::write(&mut buf, mounts, color, &args_with(cols)).unwrap();
        String::from_utf8(buf.into_inner()).unwrap()
    }

    fn args_with(cols: &str) -> Args {
        Args::try_parse_from(["dysk", "--color", "no", "--ascii", "-c", cols]).unwrap()
    }

    fn assert_terminal_safe(out: &str) {
        for c in ['\u{1b}', '\u{7}', '\u{7f}', '\u{9d}', '\u{9b}'] {
            assert!(!out.contains(c), "output contains {c:?}: {out:?}");
        }
    }

    fn mount(
        fs: &str,
        mount_point: &str,
        hostile: bool,
    ) -> Mount {
        let (label, options, uuid, part_uuid, fs_type) = if hostile {
            (
                Some("lab\u{9d}el".to_string()),
                vec![
                    MountOption::new("rw", None),
                    MountOption::new("compress", Some("zstd\u{1b}:3")),
                ],
                Some("uu\u{7f}id".to_string()),
                Some("part\u{1b}".to_string()),
                "fu\u{9b}se".to_string(),
            )
        } else {
            (
                None,
                vec![MountOption::new("rw", None)],
                None,
                None,
                "ext4".to_string(),
            )
        };
        Mount {
            info: MountInfo {
                id: Some(1),
                parent: Some(0),
                dev: device_id(),
                root: "/".into(),
                mount_point: mount_point.into(),
                options,
                fs: fs.to_string(),
                fs_type,
                bound: false,
            },
            fs_label: label,
            disk: None,
            stats: Err(StatsError::Excluded),
            uuid,
            part_uuid,
        }
    }

    fn device_id() -> DeviceId {
        #[cfg(windows)]
        {
            DeviceId::new(0x1234_5678)
        }
        #[cfg(not(windows))]
        {
            DeviceId::new(8, 16)
        }
    }
}
