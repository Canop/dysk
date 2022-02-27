mod args;
mod col;
mod cols;
mod json;
mod list_cols;
mod normal;
mod order;
mod sorting;
mod table;
mod units;

use {
    crate::{
        args::*,
        normal::*,
    },
    std::{
        fs,
        os::unix::fs::MetadataExt,
    },
};

fn main() {
    let args: Args = argh::from_env();
    if args.version {
        println!("lfs {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.list_cols {
        list_cols::print(args.color());
        return;
    }
    let mut mounts = match lfs_core::read_mounts() {
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
        let md = match fs::metadata(path) {
            Ok(md) => md,
            Err(e) => {
                eprintln!("Can't read {:?} : {}", path, e);
                return;
            }
        };
        let dev = lfs_core::DeviceId::from(md.dev());
        mounts.retain(|m| m.info.dev == dev);
    }
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json::output_value(&mounts, args.units)).unwrap()
        );
        return;
    }
    if mounts.is_empty() {
        println!("no mount to display - try\n    lfs -a");
        return;
    }
    args.sort.sort(&mut mounts);
    table::print(&mounts, args.color(), &args);
}

