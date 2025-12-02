use {
    lfs_core::Mount,
    std::path::Path,
};

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
        || m.is_remote()
    )
    && m.disk.as_ref().map_or(true, |d| !d.image) // not real
    && !m.info.bound // removing bound mounts
    && m.info.fs_type != "squashfs" // quite ad-hoc...
    && !is_system_path(&m.info.mount_point)
}

#[cfg(target_os = "macos")]
fn is_system_path(path: &Path) -> bool {
    let Some(path) = path.to_str() else {
        return false;
    };
    if path == "/" {
        return true;
    }
    if path.starts_with("/System") && !path.starts_with("/System/Volumes/Data") {
        return true;
    }
    if path.starts_with("/Library/Developer") {
        return true;
    }
    false
}

#[cfg(target_os = "linux")]
fn is_system_path(path: &Path) -> bool {
    path.starts_with("/boot")
}

#[cfg(target_os = "windows")]
fn is_system_path(_path: &Path) -> bool {
    false
}
