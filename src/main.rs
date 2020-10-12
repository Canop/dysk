
mod device_id;
mod error;
mod mount;
mod fmt_mount;
mod sys;

use argh::FromArgs;

#[derive(FromArgs)]
/// Print information on your filesystems
struct Args {
    /// whether to show all mount points
    #[argh(switch, short='a')]
    all: bool,
}

fn main() -> crate::error::Result<()>  {
    let args:Args = argh::from_env();
    let mut mounts = mount::read_all()?;
    if !args.all {
        mounts.retain(|m| m.size() > 0);
    }
    mounts.sort_by_key(|m| u64::MAX-m.size());
    fmt_mount::print(&mounts)
}
