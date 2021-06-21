mod fmt_mount;
mod json;

use argh::FromArgs;

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
