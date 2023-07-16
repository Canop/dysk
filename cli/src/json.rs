use {
    crate::units::Units,
    lfs_core::*,
    serde_json::{json, Value},
};

pub fn output_value(mounts: &[&Mount], units: Units) -> Value {
    Value::Array(
        mounts
            .iter()
            .map(|mount| {
                let stats = mount.stats().map(|s| {
                    let inodes = s.inodes.as_ref().map(|inodes| {
                        json!({
                            "files": inodes.files,
                            "free": inodes.ffree,
                            "avail": inodes.favail,
                            "used-percent": format!("{:.0}%", 100.0*inodes.use_share()),
                        })
                    });
                    json!({
                        "bsize": s.bsize,
                        "blocks": s.blocks,
                        "bfree": s.bfree,
                        "bavail": s.bavail,
                        "size": units.fmt(s.size()),
                        "used": units.fmt(s.used()),
                        "used-percent": format!("{:.0}%", 100.0*s.use_share()),
                        "available": units.fmt(s.available()),
                        "inodes": inodes,
                    })
                });
                let disk = mount.disk.as_ref().map(|d| {
                    json!({
                        "type": d.disk_type(),
                        "rotational": d.rotational,
                        "removable": d.removable,
                        "crypted": d.crypted,
                        "ram": d.ram,
                    })
                });
                json!({
                    "id": mount.info.id,
                    "dev": {
                        "major": mount.info.dev.major,
                        "minor": mount.info.dev.minor,
                    },
                    "fs": mount.info.fs,
                    "fs-label": mount.fs_label,
                    "fs-type": mount.info.fs_type,
                    "mount-point": mount.info.mount_point,
                    "disk": disk,
                    "stats": stats,
                    "bound": mount.info.bound,
                    "remote": mount.info.is_remote(),
                    "unreachable": mount.is_unreachable(),
                })
            })
            .collect(),
    )
}
