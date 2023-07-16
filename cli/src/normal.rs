use lfs_core::Mount;

/// Determine whether the mounted filesystem is "normal", which
/// means it should be listed in standard
pub fn is_normal(m: &Mount) -> bool {
    (
        m.stats().is_some()
        || m.is_unreachable()
    )
    && (
        m.disk.is_some() // by default only fs with disks are shown
        || m.info.fs_type == "zfs" // unless it's zfs - see https://github.com/Canop/dysk/issues/32
        || m.info.is_remote()
    )
    && !m.info.bound // removing bound mounts
    && m.info.fs_type != "squashfs" // quite ad-hoc...
}
