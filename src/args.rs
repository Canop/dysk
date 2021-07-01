
use {
    argh::FromArgs,
    std::path::PathBuf,
};

#[derive(FromArgs)]
/// List your filesystems.
///
/// All units are SI.
///
/// Source at https://github.com/Canop/lfs
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

    /// output as JSON
    #[argh(switch, short = 'j')]
    pub json: bool,

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
