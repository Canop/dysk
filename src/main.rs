mod fmt_mount;
mod json;

use {
    argh::FromArgs,
    std::{
        fs,
        os::unix::fs::MetadataExt,
        path::PathBuf,
    },
};

#[derive(FromArgs)]
/// List your filesystems.
///
/// All units are SI.
///
/// Source at https://github.com/Canop/lfs
struct Args {
    /// print the version
    #[argh(switch, short = 'v')]
    version: bool,

    /// whether to show all mount points
    #[argh(switch, short = 'a')]
    all: bool,

    /// output as JSON
    #[argh(switch, short = 'j')]
    json: bool,

    #[argh(positional)]
    /// if a path is provided, only the device holding this path will be shown
    pub path: Option<PathBuf>,
}

fn main() -> lfs_core::Result<()> {
    let args: Args = argh::from_env();
    if args.version {
        println!("lfs {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let mut mounts = lfs_core::read_mounts()?;
    if !args.all {
        mounts.retain(
            |m| m.disk.is_some() && m.info.fs_type != "squashfs", // quite ad-hoc...
        );
    }
    if let Some(path) = &args.path {
        let md = fs::metadata(path)?;
        let dev = lfs_core::DeviceId::from(md.dev());
        mounts.retain(|m| m.info.dev == dev);
    }
    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json::output_value(&mounts)).unwrap()
        );
        return Ok(());
    }
    if mounts.is_empty() {
        println!("no disk was found - try\n    lfs -a");
        Ok(())
    } else {
        mounts.sort_by_key(|m| u64::MAX - m.size());
        fmt_mount::print(&mounts)
    }
}
