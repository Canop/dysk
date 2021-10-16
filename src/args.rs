use {
    crate::units::Units,
    argh::FromArgs,
    std::path::PathBuf,
};

#[derive(FromArgs)]
/// List your filesystems.
///
/// Source and doc at https://github.com/Canop/lfs
pub struct Args {
    /// print the version
    #[argh(switch, short = 'v')]
    pub version: bool,

    #[argh(option, default = "Default::default()")]
    /// color: 'yes', 'no' or 'auto' (auto should be good in most cases)
    pub color: BoolArg,

    /// whether to show all mount points
    #[argh(switch, short = 'a')]
    pub all: bool,

    /// whether to show labels in the table
    #[argh(switch, short = 'l')]
    pub labels: bool,

    /// output as JSON
    #[argh(switch, short = 'j')]
    pub json: bool,

    #[argh(option, default = "Default::default()")]
    /// units: 'SI' (default) or 'binary'
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
            _ => Err(format!("Illegal value: {:?} - valid values are 'SI' and 'binary'", value)),
        }
    }
}
