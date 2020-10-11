
mod device_id;
mod error;
mod mount;
mod fmt_mount;
mod sys;

fn main() -> crate::error::Result<()>  {
    let mut mounts = mount::read_all()?;
    mounts.retain(|m| m.size() > 0);
    mounts.sort_by_key(|m| u64::MAX-m.size());
    fmt_mount::print(&mounts)
}
