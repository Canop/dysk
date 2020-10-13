
mod fmt_mount;

use argh::FromArgs;

#[derive(FromArgs)]
/// Print information on your filesystems
struct Args {
    /// whether to show all mount points
    #[argh(switch, short='a')]
    all: bool,
}

fn main() -> lfs_core::Result<()>  {
    let args:Args = argh::from_env();
    let mut mounts = lfs_core::read_mounts()?;
    if !args.all {
        mounts.retain(|m|
            m.disk.is_some()
            && m.info.fs_type != "squashfs" // quite ad-hoc...
        );
    }
    if mounts.is_empty() {
        println!("no disk was found - try\n    lfs -a");
        Ok(())
    } else {
        mounts.sort_by_key(|m| u64::MAX-m.size());
        fmt_mount::print(&mounts)
    }
}
