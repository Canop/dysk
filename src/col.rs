use {
    std::{
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
    Disk "disk" "dsk": "disk" default,
    Used "used": "used" default,
    Use "use": "use%" default,
    UsePercent "use_percent": "use%",
    Free "free": "free" default,
    Size "size": "size" default,
    InodesUsed "inodes_used" "iused": "used inodes",
    InodesUse "inodes" "ino" "inodes_use" "iuse": "inodes%",
    InodesUsePercent "inodes_use_percent" "iuse_percent": "inodes%",
    InodesFree "inodes_free" "ifree": "free inodes",
    InodesCount "inodes_total" "inodes_count" "itotal": "inodes total",
    MountPoint "mount" "mount_point" "mp": "mount point" default,
);

impl Col {
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
            Self::Filesystem => Alignment::Left,
            Self::Label => Alignment::Left,
            Self::Type => Alignment::Center,
            Self::Disk => Alignment::Center,
            Self::Used => Alignment::Right,
            Self::Use => Alignment::Right,
            Self::UsePercent => Alignment::Right,
            Self::Free => Alignment::Right,
            Self::Size => Alignment::Right,
            Self::InodesUsed => Alignment::Right,
            Self::InodesUse => Alignment::Right,
            Self::InodesUsePercent => Alignment::Right,
            Self::InodesFree => Alignment::Right,
            Self::InodesCount => Alignment::Right,
            Self::MountPoint => Alignment::Left,
        }
    }
    pub fn description(self) -> &'static str {
        match self {
            Self::Id => "mount point id",
            Self::Dev => "device id",
            Self::Filesystem => "filesystem",
            Self::Label => "volume label",
            Self::Type => "filesystem type",
            Self::Disk => "storage type",
            Self::Used => "size used",
            Self::Use => "usage graphical view",
            Self::UsePercent => "percentage of blocks used",
            Self::Free => "free bytes",
            Self::Size => "total size",
            Self::InodesUsed => "number of inodes used",
            Self::InodesUse => "graphical view of inodes usage",
            Self::InodesUsePercent => "percentage of inodes used",
            Self::InodesFree => "number of free inodes",
            Self::InodesCount => "total count of inodes",
            Self::MountPoint => "mount point",
        }
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
        write!(f, "{:?} can't be parsed as a column; expected one of ", self.raw)?;
        let mut names = ALL_COLS.iter().map(|c| c.name()).peekable();
        write!(f, "{:?}", names.next().unwrap())?;
        loop {
            if let Some(name) = names.next() {
                if names.peek().is_none() {
                    write!(f, ", or {:?}", name)?;
                    break;
                } else {
                    write!(f, ", {:?}", name)?;
                }
            }
        }
        Ok(())
    }
}
impl std::error::Error for ParseColError {}

