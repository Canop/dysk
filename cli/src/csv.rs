use {
    crate::{
        Args,
        col::Col,
    },
    lfs_core::*,
    std::{
        fmt::Display,
        io::Write,
    },
};

/// Utility to write in CSV
struct Csv<W: Write> {
    separator: char,
    w: W,
}

impl<W: Write> Csv<W> {
    pub fn new(
        separator: char,
        w: W,
    ) -> Self {
        Self { separator, w }
    }
    pub fn cell<D: Display>(
        &mut self,
        content: D,
    ) -> Result<(), std::io::Error> {
        let s = content.to_string();
        let needs_quotes = s.contains(self.separator) || s.contains('"') || s.contains('\n');
        if needs_quotes {
            write!(self.w, "\"")?;
            for c in s.chars() {
                if c == '"' {
                    write!(self.w, "\"\"")?;
                } else {
                    write!(self.w, "{}", c)?;
                }
            }
            write!(self.w, "\"")?;
        } else {
            write!(self.w, "{}", s)?;
        }
        write!(self.w, "{}", self.separator)
    }
    pub fn cell_opt<D: Display>(
        &mut self,
        content: Option<D>,
    ) -> Result<(), std::io::Error> {
        if let Some(c) = content {
            self.cell(c)
        } else {
            write!(self.w, "{}", self.separator)
        }
    }
    pub fn end_line(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.w)
    }
}

pub fn write<W: Write>(
    w: &mut W,
    mounts: &[&Mount],
    args: &Args,
) -> std::io::Result<()> {
    let units = args.units;
    let mut csv = Csv::new(args.csv_separator, w);
    for col in args.cols.cols() {
        csv.cell(col.title())?;
    }
    csv.end_line()?;
    for mount in mounts {
        for col in args.cols.cols() {
            match col {
                Col::Id => csv.cell_opt(mount.info.id),
                Col::Dev => csv.cell(&mount.info.dev),
                Col::Filesystem => csv.cell(&mount.info.fs),
                Col::Label => csv.cell_opt(mount.fs_label.as_ref()),
                Col::Type => csv.cell(&mount.info.fs_type),
                Col::Remote => csv.cell(if mount.is_remote() { "yes" } else { "no" }),
                Col::Disk => csv.cell_opt(mount.disk.as_ref().map(|d| d.disk_type())),
                Col::Used => csv.cell_opt(mount.stats().map(|s| units.fmt(s.used()))),
                Col::Use => csv.cell_opt(mount.stats().map(|s| s.use_share())),
                Col::UsePercent => csv.cell_opt(
                    mount
                        .stats()
                        .map(|s| format!("{:.0}%", 100.0 * s.use_share())),
                ),
                Col::Free => csv.cell_opt(mount.stats().map(|s| units.fmt(s.available()))),
                Col::FreePercent => csv.cell_opt(
                    mount
                        .stats()
                        .map(|s| format!("{:.0}%", 100.0 * (1.0 - s.use_share()))),
                ),
                Col::Size => csv.cell_opt(mount.stats().map(|s| units.fmt(s.size()))),
                Col::InodesUsed => csv.cell_opt(mount.inodes().map(|i| i.used())),
                Col::InodesUse => csv.cell_opt(mount.inodes().map(|i| i.use_share())),
                Col::InodesUsePercent => csv.cell_opt(
                    mount
                        .inodes()
                        .map(|i| format!("{:.0}%", 100.0 * i.use_share())),
                ),
                Col::InodesFree => csv.cell_opt(mount.inodes().map(|i| i.favail)),
                Col::InodesCount => csv.cell_opt(mount.inodes().map(|i| i.files)),
                Col::MountPoint => csv.cell(mount.info.mount_point.to_string_lossy()),
                Col::Uuid => csv.cell(mount.uuid.as_ref().map_or("", |v| v)),
                Col::PartUuid => csv.cell(mount.part_uuid.as_ref().map_or("", |v| v)),
                Col::MountOptions => csv.cell(mount.info.options_string()),
                Col::CompressLevel => csv.cell_opt(mount.info.option_value("compress")),
            }?;
        }
        csv.end_line()?;
    }
    Ok(())
}

#[test]
fn test_csv() {
    use std::io::Cursor;
    let mut w = Cursor::new(Vec::new());
    let mut csv = Csv::new(';', &mut w);
    csv.cell("1;2;3").unwrap();
    csv.cell("\"").unwrap();
    csv.cell("").unwrap();
    csv.end_line().unwrap();
    csv.cell(3).unwrap();
    let s = String::from_utf8(w.into_inner()).unwrap();
    assert_eq!(
        s,
        r#""1;2;3";"""";;
3;"#,
    );
}
