use {
    std::{
        fmt,
        str::FromStr,
    },
    termimad::minimad::Alignment,
};

/// A column of the lfs table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Col {
    Id,
    Dev,
    Filesystem, // alias fs
    Label,
    Disk,
    Type,
    Used,
    Use,
    Free,
    Size,
    InodesFree,
    InodesUsed,
    InodesUse, // alias inode
    InodesCount,
    MountPoint, // alias mount
}

pub static DEFAULT_COLS: &[Col] = &[
    Col::Filesystem,
    Col::Disk,
    Col::Type,
    Col::Used,
    Col::Use,
    Col::Free,
    Col::Size,
    Col::MountPoint,
];

pub static ALL_COLS: &[Col] = &[
    Col::Id,
    Col::Dev,
    Col::Filesystem,
    Col::Label,
    Col::Disk,
    Col::Type,
    Col::Used,
    Col::Use,
    Col::Free,
    Col::Size,
    Col::InodesUsed,
    Col::InodesUse,
    Col::InodesFree,
    Col::InodesCount,
    Col::MountPoint,
];

impl Col {
    pub fn title(self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Dev => "dev",
            Self::Filesystem => "filesystem",
            Self::Label => "label",
            Self::Disk => "disk",
            Self::Type => "type",
            Self::Used => "used",
            Self::Use => "use%",
            Self::Free => "free",
            Self::Size => "size",
            Self::InodesUsed => "used inodes",
            Self::InodesUse => "inodes%",
            Self::InodesFree => "free inodes",
            Self::InodesCount => "inodes total",
            Self::MountPoint => "mount point",
        }
    }
    pub fn header_align(self) -> Alignment {
        match self {
            Self::Label => Alignment::Left,
            Self::MountPoint => Alignment::Left,
            _ => Alignment::Center,
        }
    }
    pub fn content_align(self) -> Alignment {
        match self {
            Self::Id => Alignment::Right,
            Self::Dev => Alignment::Center,
            Self::Filesystem => Alignment::Center,
            Self::Label => Alignment::Left,
            Self::Disk => Alignment::Center,
            Self::Type => Alignment::Center,
            Self::Used => Alignment::Right,
            Self::Use => Alignment::Right,
            Self::Free => Alignment::Right,
            Self::Size => Alignment::Right,
            Self::InodesUsed => Alignment::Right,
            Self::InodesUse => Alignment::Center,
            Self::InodesFree => Alignment::Right,
            Self::InodesCount => Alignment::Right,
            Self::MountPoint => Alignment::Left,
        }
    }
}

impl fmt::Display for Col {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title())
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} can't be parsed as a column", self.raw)
    }
}
impl std::error::Error for ParseColError {}

impl FromStr for Col {
    type Err = ParseColError;
    fn from_str(s: &str) -> Result<Self, ParseColError> {
        match s {
            "id" => Ok(Self::Id),
            "dev" | "device" => Ok(Self::Dev),
            "fs" | "filesystem" => Ok(Self::Filesystem),
            "label" => Ok(Self::Label),
            "disk" => Ok(Self::Disk),
            "type" => Ok(Self::Type),
            "used" => Ok(Self::Used),
            "use" => Ok(Self::Use),
            "free" => Ok(Self::Free),
            "size" => Ok(Self::Size),
            "inodes_free" => Ok(Self::InodesFree),
            "inodes_used" => Ok(Self::InodesUsed),
            "inodes" | "inode_use" | "inodes_use" => Ok(Self::InodesUse),
            "inode_count" | "inodes_count" => Ok(Self::InodesCount),
            "mount" | "mountpoint" => Ok(Self::MountPoint),
            _ => Err(ParseColError::new(s)),
        }
    }
}
