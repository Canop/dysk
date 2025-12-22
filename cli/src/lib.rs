pub mod args;
pub mod col;
pub mod col_expr;
pub mod cols;
pub mod csv;
pub mod filter;
pub mod help;
pub mod json;
pub mod list_cols;
pub mod normal;
pub mod order;
pub mod sorting;
pub mod table;
pub mod timeout;
pub mod units;

use {
    crate::{
        args::*,
        normal::*,
    },
    clap::Parser,
    std::io::{
        self,
        Write,
    },
};

/// Print according to launch arguments
///
/// # Errors
/// Returns an `io::Error` if writing to stdout fails
#[allow(clippy::match_like_matches_macro)]
pub fn run() -> io::Result<()> {
    let mut w = io::stdout();
    let args = Args::parse();
    if args.version {
        return writeln!(&mut w, "dysk {}", env!("CARGO_PKG_VERSION"));
    }
    if args.help {
        help::print(args.ascii);
        if args.color() {
            csi_reset();
        }
        return Ok(());
    }
    if args.list_cols {
        list_cols::write(&mut w, args.color(), args.ascii)?;
        if args.color() {
            csi_reset();
        }
        return Ok(());
    }
    let mut options =
        lfs_core::ReadOptions::default()
        .remote_stats(args.remote_stats.unwrap_or_else(|| true));
    if let Some(timeout) = args.timeout {
        options = options.stats_timeout(timeout.as_duration());
    }
    if let Some(strategy) = &args.strategy {
        match strategy.parse() {
            Ok(strategy) => {
                options = options.strategy(strategy);
            }
            Err(_) => {
                eprintln!("Ignoring unrecognized strategy");
            }
        }
    }
    let mut mounts = match lfs_core::read_mounts(&options) {
        Ok(mounts) => mounts,
        Err(e) => {
            eprintln!("Error reading mounts: {}", e);
            return Ok(());
        }
    };
    if !args.all {
        mounts.retain(is_normal);
    }
    if let Some(path) = &args.path {
        let dev = match lfs_core::DeviceId::of_path(path) {
            Ok(dev) => dev,
            Err(e) => {
                eprintln!("Error getting device of path {}: {}", path.display(), e);
                return Ok(());
            }
        };
        mounts.retain(|m| m.info.dev == dev);
    }
    args.sort.sort(&mut mounts);
    let mounts = match args.filter.clone().unwrap_or_default().filter(&mounts) {
        Ok(mounts) => mounts,
        Err(e) => {
            eprintln!("Error in filter evaluation: {}", e);
            return Ok(());
        }
    };
    if args.csv {
        return csv::write(&mut w, &mounts, &args);
    }
    if args.json {
        return writeln!(
            &mut w,
            "{}",
            serde_json::to_string_pretty(&json::output_value(&mounts, args.units)).unwrap()
        );
    }
    if mounts.is_empty() {
        return writeln!(&mut w, "no mount to display - try\n    dysk -a");
    }
    table::write(&mut w, &mounts, args.color(), &args)?;
    if args.color() {
        csi_reset();
    }
    Ok(())
}

/// output a Reset CSI sequence
fn csi_reset() {
    print!("\u{1b}[0m");
}

