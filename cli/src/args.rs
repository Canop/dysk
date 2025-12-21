use {
    crate::{
        cols::Cols,
        filter::Filter,
        sorting::Sorting,
        timeout::Timeout,
        units::Units,
    },
    clap::{
        Parser,
        ValueEnum,
    },
    termimad::crossterm::tty::IsTty,
};

/// List your filesystems.
///
/// Documentation at https://dystroy.org/dysk
#[derive(Debug, Parser)]
#[command(
    author,
    about,
    name = "dysk",
    disable_version_flag = true,
    version,
    disable_help_flag = true
)]
pub struct Args {
    /// print help information
    #[arg(long)]
    pub help: bool,

    /// print the version
    #[arg(long)]
    pub version: bool,

    /// show all mount points
    #[arg(short, long)]
    pub all: bool,

    /// whether to have styles and colors
    #[arg(long, default_value = "auto", value_name = "color")]
    pub color: TriBool,

    /// use only ASCII characters for table rendering
    #[arg(long)]
    pub ascii: bool,

    /// fetch stats of remote volumes
    #[arg(long, default_value = "auto", value_name = "choice")]
    pub remote_stats: TriBool,

    /// list the column names which can be used in -s, -f, or -c
    #[arg(long)]
    pub list_cols: bool,

    /// columns, eg `-c +inodes` or `-c id+dev+default`
    #[arg(
        short,
        long,
        default_value = "fs+type+disk+used+use+free+size+mp",
        value_name = "columns"
    )]
    pub cols: Cols,

    /// filter, eg `-f '(size<35G | remote=false) & type=xfs'`
    #[arg(short, long, value_name = "expr")]
    pub filter: Option<Filter>,

    /// sort, eg `inodes`, `type-desc`, or `size-asc`
    #[arg(short, long, default_value = "size", value_name = "sort")]
    pub sort: Sorting,

    /// units: `SI` (SI norm), `binary` (1024 based), or `bytes` (raw number)
    #[arg(short, long, default_value = "SI", value_name = "unit")]
    pub units: Units,

    /// output as JSON
    #[arg(short, long)]
    pub json: bool,

    /// output as CSV
    #[arg(long)]
    pub csv: bool,

    /// CSV separator
    #[arg(long, default_value = ",", value_name = "sep")]
    pub csv_separator: char,

    /// strategy to use to find information, when several are available
    #[arg(long, hide = true)]
    pub strategy: Option<String>,

    /// timeout to use for some operations, eg `10s` or `5ms`
    #[arg(long)]
    pub timeout: Option<Timeout>,

    /// if provided, only the device holding this path will be shown
    pub path: Option<std::path::PathBuf>,
}

/// This is an Option<bool> but I didn't find any way to configure
/// clap to parse an Option<T> as I want
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriBool {
    Auto,
    Yes,
    No,
}
impl TriBool {
    pub fn unwrap_or_else<F>(
        self,
        f: F,
    ) -> bool
    where
        F: FnOnce() -> bool,
    {
        match self {
            Self::Auto => f(),
            Self::Yes => true,
            Self::No => false,
        }
    }
}

impl Args {
    pub fn color(&self) -> bool {
        self.color.unwrap_or_else(|| std::io::stdout().is_tty())
    }
}
