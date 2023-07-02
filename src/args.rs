use {
    crate::{
        cols::Cols,
        filter::Filter,
        units::Units,
        sorting::Sorting,
    },
    crossterm::tty::IsTty,
    argh::FromArgs,
    std::path::PathBuf,
};

#[derive(FromArgs)]
/// List your filesystems.
///
/// Documentation at https://dystroy.org/dysk
pub struct Args {
    /// print the version
    #[argh(switch, short = 'v')]
    pub version: bool,

    #[argh(option, default = "Default::default()")]
    /// color: 'yes', 'no' or 'auto'
    pub color: BoolArg,

    /// show all mount points
    #[argh(switch, short = 'a')]
    pub all: bool,

    /// list the available column names
    #[argh(switch)]
    pub list_cols: bool,

    /// fetch stats of remote volumes, 'yes', 'no' or 'auto'
    #[argh(option, default = "Default::default()")]
    pub remote_stats: BoolArg,

    /// columns, eg `-c +inodes` or `-c id+dev+default`
    #[argh(option, default = "Default::default()", short = 'c')]
    pub cols: Cols,

    /// filter, eg `-f '(size<35G | remote=false) & type=xfs'`
    #[argh(option, default = "Default::default()", short = 'f')]
    pub filter: Filter,

    /// sort, eg `-s inodes` or `-s size-asc`
    #[argh(option, default = "Default::default()", short = 's')]
    pub sort: Sorting,

    /// output as CSV
    #[argh(switch)]
    pub csv: bool,

    /// CSV separator (default: ',')
    #[argh(option, default = "','")]
    pub csv_separator: char,

    /// output as JSON
    #[argh(switch, short = 'j')]
    pub json: bool,

    #[argh(option, default = "Default::default()")]
    /// units: 'SI' (default), 'binary' (1024 based), or 'bytes' (the raw number)
    pub units: Units,

    #[argh(positional)]
    /// if a path is provided, only the device holding this path will be shown
    pub path: Option<PathBuf>,
}

/// An optional boolean for use in Argh
#[derive(Debug, Clone, Copy, Default)]
pub struct BoolArg(Option<bool>);

impl BoolArg {
    pub fn value(self) -> Option<bool> {
        self.0
    }
}

impl argh::FromArgValue for BoolArg {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_ref() {
            "auto" => Ok(BoolArg(None)),
            "yes" => Ok(BoolArg(Some(true))),
            "no" => Ok(BoolArg(Some(false))),
            _ => Err(format!("Illegal value: {:?}", value)),
        }
    }
}

impl argh::FromArgValue for Units {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_ref() {
            "si" => Ok(Self::Si),
            "binary" => Ok(Self::Binary),
            "bytes" => Ok(Self::Bytes),
            _ => Err(format!("Illegal value: {:?} - valid values are 'SI', 'binary', and 'bytes'", value)),
        }
    }
}
impl Args {
    pub fn color(&self) -> bool {
        self.color.value()
            .unwrap_or_else(|| std::io::stdout().is_tty())
    }
}
