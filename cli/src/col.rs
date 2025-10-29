use {
    crate::order::Order,
    lfs_core::Mount,
    std::{
        cmp::Ordering,
        fmt,
        str::FromStr,
    },
    termimad::minimad::Alignment,
};

macro_rules! col_enum {
    (@just_variant $variant:ident $discarded:ident) => {
        Col::$variant
    };
    ($($variant:ident $name:literal $($alias:literal)* : $title:literal $($def:ident)*,)*) => {
        /// A column of the lfs table.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Col {
            $($variant,)*
        }
        pub static ALL_COLS: &[Col] = &[
            $(Col::$variant,)*
        ];
        pub static DEFAULT_COLS: &[Col] = &[
            $(
                $(col_enum!(@just_variant $variant $def),)*
            )*
        ];
        impl FromStr for Col {
            type Err = ParseColError;
            fn from_str(s: &str) -> Result<Self, ParseColError> {
                match s {
                    $(
                        $name => Ok(Self::$variant),
                        $(
                            $alias => Ok(Self::$variant),
                        )*
                    )*
                    _ => Err(ParseColError::new(s)),
                }
            }
        }
        impl fmt::Display for Col {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Self::$variant => write!(f, "{}", self.title()),
                    )*
                }
            }
        }
        impl Col {
            pub fn name(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $name,
                    )*
                }
            }
            pub fn title(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $title,
                    )*
                }
            }
            pub fn aliases(self) -> &'static [&'static str] {
                match self {
                    $(
                        Self::$variant => &[$($alias,)*],
                    )*
                }
            }
            pub fn is_default(self) -> bool {
                DEFAULT_COLS.contains(&self)
            }
        }
    };
}

// definition of all columns and their names
// in the --cols definition
col_enum!(
    // syntax:
    // Variant name [aliases]: title [default]
    Id "id": "id",
    Dev "dev" "device" "device_id": "dev",
    Filesystem "fs" "filesystem": "filesystem" default,
    Label "label": "label",
    Type "type": "type" default,
    Remote "remote" "rem": "remote",
    Disk "disk" "dsk": "disk" default,
    Used "used": "used" default,
    Use "use": "use" default,
    UsePercent "use_percent": "use%",
    Free "free": "free" default,
    FreePercent "free_percent": "free%",
    Size "size": "size" default,
    InodesUsed "inodes_used" "iused": "used inodes",
    InodesUse "inodes" "ino" "inodes_use" "iuse": "inodes",
    InodesUsePercent "inodes_use_percent" "iuse_percent": "inodes%",
    InodesFree "inodes_free" "ifree": "free inodes",
    InodesCount "inodes_total" "inodes_count" "itotal": "inodes total",
    MountPoint "mount" "mount_point" "mp": "mount point" default,
    Uuid "uuid": "UUID",
    PartUuid "partuuid" "part_uuid": "PARTUUID",
    MountOptions "options" "mount_options": "mount options",
    CompressLevel "compress" "compress_level": "compress",
);

impl Col {
    pub fn header_align(self) -> Alignment {
        match self {
            Self::Label => Alignment::Left,
            Self::MountPoint => Alignment::Left,
            Self::MountOptions => Alignment::Left,
            _ => Alignment::Center,
        }
    }
    pub fn content_align(self) -> Alignment {
        match self {
            Self::Id => Alignment::Right,
            Self::Dev => Alignment::Center,
            Self::Filesystem => Alignment::Left,
            Self::Label => Alignment::Left,
            Self::Type => Alignment::Center,
            Self::Remote => Alignment::Center,
            Self::Disk => Alignment::Center,
            Self::Used => Alignment::Right,
            Self::Use => Alignment::Right,
            Self::UsePercent => Alignment::Right,
            Self::Free => Alignment::Right,
            Self::FreePercent => Alignment::Right,
            Self::Size => Alignment::Right,
            Self::InodesUsed => Alignment::Right,
            Self::InodesUse => Alignment::Right,
            Self::InodesUsePercent => Alignment::Right,
            Self::InodesFree => Alignment::Right,
            Self::InodesCount => Alignment::Right,
            Self::MountPoint => Alignment::Left,
            Self::Uuid => Alignment::Left,
            Self::PartUuid => Alignment::Left,
            Self::MountOptions => Alignment::Left,
            Self::CompressLevel => Alignment::Center,
        }
    }
    pub fn description(self) -> &'static str {
        match self {
            Self::Id => "mount point id",
            Self::Dev => "device id",
            Self::Filesystem => "filesystem",
            Self::Label => "volume label",
            Self::Type => "filesystem type",
            Self::Remote => "whether it's a remote filesystem",
            Self::Disk => "storage type",
            Self::Used => "size used",
            Self::Use => "usage graphical view",
            Self::UsePercent => "percentage of blocks used",
            Self::Free => "free bytes",
            Self::FreePercent => "percentage of free blocks",
            Self::Size => "total size",
            Self::InodesUsed => "number of inodes used",
            Self::InodesUse => "graphical view of inodes usage",
            Self::InodesUsePercent => "percentage of inodes used",
            Self::InodesFree => "number of free inodes",
            Self::InodesCount => "total count of inodes",
            Self::MountPoint => "mount point",
            Self::Uuid => "filesystem UUID",
            Self::PartUuid => "partition UUID",
            Self::MountOptions => "mount options (linux only)",
            Self::CompressLevel => "compress algo/level",
        }
    }
    pub fn comparator(self) -> impl for<'a, 'b> FnMut(&'a Mount, &'b Mount) -> Ordering {
        match self {
            Self::Id => |a: &Mount, b: &Mount| a.info.id.cmp(&b.info.id),
            Self::Dev => |a: &Mount, b: &Mount| a.info.dev.cmp(&b.info.dev),
            Self::Filesystem => |a: &Mount, b: &Mount| a.info.fs.cmp(&b.info.fs),
            Self::Label => |a: &Mount, b: &Mount| match (&a.fs_label, &b.fs_label) {
                (Some(a), Some(b)) => a.cmp(b),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            Self::Type => |a: &Mount, b: &Mount| a.info.fs_type.cmp(&b.info.fs_type),
            Self::Remote => |a: &Mount, b: &Mount| a.is_remote().cmp(&b.is_remote()),
            Self::Disk => |a: &Mount, b: &Mount| match (&a.disk, &b.disk) {
                (Some(a), Some(b)) => a
                    .disk_type()
                    .to_lowercase()
                    .cmp(&b.disk_type().to_lowercase()),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Self::Used => |a: &Mount, b: &Mount| match (&a.stats(), &b.stats()) {
                (Some(a), Some(b)) => a.used().cmp(&b.used()),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Self::Use | Self::UsePercent => |a: &Mount, b: &Mount| match (&a.stats(), &b.stats()) {
                // the 'use' column shows the percentage of used blocks, so it makes sense
                // to sort by use_share for it
                // SAFETY: use_share() doesn't return NaN
                (Some(a), Some(b)) => a.use_share().partial_cmp(&b.use_share()).unwrap(),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Self::Free => |a: &Mount, b: &Mount| match (&a.stats(), &b.stats()) {
                (Some(a), Some(b)) => a.available().cmp(&b.available()),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Self::FreePercent => |a: &Mount, b: &Mount| match (&a.stats(), &b.stats()) {
                (Some(a), Some(b)) => b.use_share().partial_cmp(&a.use_share()).unwrap(),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Self::Size => |a: &Mount, b: &Mount| match (&a.stats(), &b.stats()) {
                (Some(a), Some(b)) => a.size().cmp(&b.size()),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Self::InodesUsed => |a: &Mount, b: &Mount| match (&a.inodes(), &b.inodes()) {
                (Some(a), Some(b)) => a.used().cmp(&b.used()),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Self::InodesUsePercent | Self::InodesUse => {
                |a: &Mount, b: &Mount| match (&a.inodes(), &b.inodes()) {
                    // SAFETY: use_share() doesn't return NaN
                    (Some(a), Some(b)) => a.use_share().partial_cmp(&b.use_share()).unwrap(),
                    (Some(_), None) => Ordering::Greater,
                    (None, Some(_)) => Ordering::Less,
                    (None, None) => Ordering::Equal,
                }
            }
            Self::InodesFree => |a: &Mount, b: &Mount| match (&a.inodes(), &b.inodes()) {
                (Some(a), Some(b)) => a.favail.cmp(&b.favail),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Self::InodesCount => |a: &Mount, b: &Mount| match (&a.inodes(), &b.inodes()) {
                (Some(a), Some(b)) => a.files.cmp(&b.files),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Self::MountPoint => |a: &Mount, b: &Mount| a.info.mount_point.cmp(&b.info.mount_point),
            Self::Uuid => |a: &Mount, b: &Mount| match (&a.uuid, &b.uuid) {
                (Some(a), Some(b)) => a.cmp(b),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            Self::PartUuid => |a: &Mount, b: &Mount| match (&a.part_uuid, &b.part_uuid) {
                (Some(a), Some(b)) => a.cmp(b),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            Self::MountOptions => |a: &Mount, b: &Mount| a.info.options_string()
                .cmp(&b.info.options_string()),
            Self::CompressLevel => |a: &Mount, b: &Mount| match (a.info.option_value("compress"), b.info.option_value("compress")) {
                (Some(a), Some(b)) => a.cmp(b),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
        }
    }
    pub fn default_sort_order(self) -> Order {
        match self {
            Self::Id => Order::Asc,
            Self::Dev => Order::Asc,
            Self::Filesystem => Order::Asc,
            Self::Label => Order::Asc,
            Self::Type => Order::Asc,
            Self::Remote => Order::Desc,
            Self::Disk => Order::Asc,
            Self::Used => Order::Asc,
            Self::Use => Order::Desc,
            Self::UsePercent => Order::Asc,
            Self::Free => Order::Asc,
            Self::FreePercent => Order::Desc,
            Self::Size => Order::Desc,
            Self::InodesUsed => Order::Asc,
            Self::InodesUse => Order::Asc,
            Self::InodesUsePercent => Order::Asc,
            Self::InodesFree => Order::Asc,
            Self::InodesCount => Order::Asc,
            Self::MountPoint => Order::Asc,
            Self::Uuid => Order::Asc,
            Self::PartUuid => Order::Asc,
            Self::MountOptions => Order::Asc,
            Self::CompressLevel => Order::Asc,
        }
    }
    pub fn default_sort_col() -> Self {
        Self::Size
    }
}

#[derive(Debug)]
pub struct ParseColError {
    /// the string which couldn't be parsed
    pub raw: String,
}
impl ParseColError {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self { raw: s.into() }
    }
}
impl fmt::Display for ParseColError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{:?} can't be parsed as a column; use 'dysk --list-cols' to see all column names",
            self.raw,
        )
    }
}
impl std::error::Error for ParseColError {}
