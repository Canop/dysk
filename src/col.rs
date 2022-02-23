use {
    std::{
        fmt,
        str::FromStr,
    },
    termimad::minimad::Alignment,
};

macro_rules! col_enum {
    ($($variant:ident $name:literal $($alias:literal)* : $title:literal ,)*) => {
        /// A column of the lfs table.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Col {
            $($variant,)*
        }
        pub static ALL_COLS: &[Col] = &[
            $(Col::$variant,)*
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
        }
    }
}

// definition of all columns and their names
// in the --cols definition
col_enum!(
    // syntax: Variant name [alias] : title
    Id "id": "id",
    Dev "dev": "dev",
    Filesystem "fs" "filesystem": "filesystem",
    Label "label": "label",
    Disk "disk": "disk",
    Type "type": "type",
    Used "used": "used",
    Use "use": "use%",
    UsePercent "use_percent": "use%",
    Free "free": "free",
    Size "size": "size",
    InodesUsed "inodes_used": "used inodes",
    InodesUse "inodes" "inodes_use": "inodes%",
    InodesUsePercent "inodes_use_percent": "inodes%",
    InodesFree "inodes_free": "free inodes",
    InodesCount "inodes_total" "inodes_count": "inodes total",
    MountPoint "mount" "mount_point": "mount point",
);

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
            Self::Disk => Alignment::Center,
            Self::Type => Alignment::Center,
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

