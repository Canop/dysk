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
pub mod units;

use {
    crate::{
        args::*,
        normal::*,
    },
    clap::Parser,
};

#[allow(clippy::match_like_matches_macro)]
pub fn run() {
    let args = Args::parse();
    if args.version {
        println!("dysk {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.help {
        help::print(args.ascii);
        csi_reset();
        return;
    }
    if args.list_cols {
        list_cols::print(args.color(), args.ascii);
        csi_reset();
        return;
    }
    let mut options =
        lfs_core::ReadOptions::default()
        .remote_stats(args.remote_stats.unwrap_or_else(|| true));
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
            return;
        }
    };
    if !args.all {
        mounts.retain(is_normal);
    }
    if let Some(path) = &args.path {
        use std::os::unix::fs::MetadataExt;
        let md = match std::fs::metadata(path) {
            Ok(md) => md,
            Err(e) => {
                eprintln!("Can't read {:?} : {}", path, e);
                return;
            }
        };
        let dev = lfs_core::DeviceId::from(md.dev());
        mounts.retain(|m| m.info.dev == dev);
    }
    args.sort.sort(&mut mounts);
    let mounts = match args.filter.clone().unwrap_or_default().filter(&mounts) {
        Ok(mounts) => mounts,
        Err(e) => {
            eprintln!("Error in filter evaluation: {}", e);
            return;
        }
    };
    if args.csv {
        csv::print(&mounts, &args).expect("writing csv failed");
        return;
    }
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json::output_value(&mounts, args.units)).unwrap()
        );
        return;
    }
    if mounts.is_empty() {
        println!("no mount to display - try\n    dysk -a");
        return;
    }
    table::print(&mounts, args.color(), &args);
    csi_reset();
}

/// output a Reset CSI sequence
fn csi_reset() {
    print!("\u{1b}[0m");
}
