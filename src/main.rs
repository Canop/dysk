mod args;
mod fmt_mount;
mod json;
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

fn main() -> lfs_core::Result<()> {
    let args: Args = argh::from_env();
    if args.version {
        println!("lfs {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let mut mounts = lfs_core::read_mounts()?;
    if !args.all {
        mounts.retain(|m|
            m.disk.is_some() // by default only fs with disks are shown
            && !m.info.bound // removing bound mounts
            && m.info.fs_type != "squashfs", // quite ad-hoc...
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
            serde_json::to_string_pretty(&json::output_value(&mounts, args.units)).unwrap()
        );
        return Ok(());
    }
    if mounts.is_empty() {
        println!("no disk was found - try\n    lfs -a");
    } else {
        mounts.sort_by_key(|m| Reverse(m.size()));
        let color = args.color.value()
            .unwrap_or_else(|| std::io::stdout().is_tty());
        fmt_mount::print(&mounts, color, &args);
    }
    Ok(())
}
