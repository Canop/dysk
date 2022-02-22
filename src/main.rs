mod args;
mod col;
mod cols;
mod json;
mod table;
mod units;

use {
    crate::args::*,
    crossterm::tty::IsTty,
    std::{
        cmp::Reverse,
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
    let mut mounts = match lfs_core::read_mounts() {
        Ok(mounts) => mounts,
        Err(e) => {
            eprintln!("Error reading mounts: {}", e);
            return;
        }
    };
    if !args.all {
        mounts.retain(|m|
            (
                m.disk.is_some() // by default only fs with disks are shown
                || m.info.fs_type == "zfs" // unless it's zfs - see https://github.com/Canop/lfs/issues/32
            )
            && !m.info.bound // removing bound mounts
            && m.info.fs_type != "squashfs", // quite ad-hoc...
        );
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
        println!("no disk was found - try\n    lfs -a");
    } else {
        mounts.sort_by_key(|m| Reverse(m.size()));
        let color = args.color.value()
            .unwrap_or_else(|| std::io::stdout().is_tty());
        table::print(&mounts, color, &args);
    }
}

