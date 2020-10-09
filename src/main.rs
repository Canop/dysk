
mod device_id;
mod error;
mod mount;
mod sys;

use {
    crate::error::*,
};

fn main() -> Result<()>  {
    let mounts = mount::read_all()?;
    println!("Found {} mounts:", mounts.len());
    for mount in mounts {
        //if mount.mount_point.starts_with("/sys/") {
        //    continue;
        //}
        println!("{:?}", mount);
    }
    Ok(())
}
