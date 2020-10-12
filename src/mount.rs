
use {
    crate::{
        device_id::DeviceId,
        error::*,
        sys,
    },
    std::{
        ffi::CString,
        mem,
        os::unix::ffi::OsStrExt,
        path::PathBuf,
        str::{FromStr, SplitWhitespace},
    },
};

/// I have no idea of the size of this id
pub type MountId = u32;

/// inode & blocs information given by statvfs
#[derive(Debug)]
pub struct Stats {
    pub bsize: u64,
    pub blocks: u64,
    pub bavail: u64,
    pub bfree: u64,
}

/// A mount point
#[derive(Debug)]
pub struct Mount {
    pub id: MountId,
    pub parent: MountId,
    pub dev: DeviceId,
    pub root: PathBuf,
    pub mount_point: PathBuf,
    pub fs: String,
    pub fs_type: String,

    /// true for HDD, false for SSD, None for unknown or not a disk
    pub rotational: Option<bool>,
    pub stats: Option<Stats>,
}

impl Mount {
    pub fn size(&self) -> u64 {
        self.stats.as_ref().map_or(0, |s| s.bsize * s.blocks)
    }
    pub fn available(&self) -> u64 {
        self.stats.as_ref().map_or(0, |s| s.bsize * s.bavail)
    }
    pub fn used(&self) -> u64 {
        self.size() - self.available()
    }
    pub fn use_share(&self) -> f64 {
        if self.size() == 0 {
            0.0
        } else {
            self.used() as f64 / (self.size() as f64)
        }
    }
    pub fn partition_name(&self) -> Option<String> {
        self.fs.strip_prefix("/dev/").map(String::from)
    }
    /// Something like "sda" or "sdb"
    pub fn disk_name(&self) -> Option<String> {
        // Q: Am I sure this is correct ?
        // A: Absolutely not!
        self.partition_name()
            .map(|pn| pn.chars().take_while(char::is_ascii_alphabetic).collect::<String>())
            .filter(|s| s.len()>2)
    }
    pub fn disk_type(&self) -> &'static str {
        match self.rotational {
            Some(true) => "HDD",
            Some(false) => "SSD",
            None => "",
        }
    }
}

fn next<'a,'b>(split: &'b mut SplitWhitespace<'a>) -> Result<&'a str> {
    split.next().ok_or(Error::UnexpectedFormat)
}
fn skip_until<'a,'b>(split: &'b mut SplitWhitespace<'a>, sep: &'static str) -> Result<()> {
    Ok(loop {
        if next(split)? == sep { break; }
    })
}

impl FromStr for Mount {
    type Err = Error;
    fn from_str(line: &str) -> Result<Self> {
        // this parsing is based on `man 5 proc`
        let mut tokens = line.split_whitespace();
        let tokens = &mut tokens;
        let id = next(tokens)?.parse()?;
        let parent = next(tokens)?.parse()?;
        let dev = next(tokens)?.parse()?;
        let root = next(tokens)?.into();
        let mount_point = PathBuf::from(next(tokens)?);
        skip_until(tokens, "-")?;
        let fs_type = next(tokens)?.to_string();
        let fs = next(tokens)?.to_string();
        // we get the free/total space info in libc::statvfs
        let c_mount_point = CString::new(mount_point.as_os_str().as_bytes()).unwrap();
        let stats = unsafe {
            let mut statvfs = mem::MaybeUninit::<libc::statvfs>::uninit();
            let code = libc::statvfs(c_mount_point.as_ptr(), statvfs.as_mut_ptr());
            match code {
                0 => {
                    // good
                    let statvfs = statvfs.assume_init();
                    Some(Stats {
                        bsize: statvfs.f_bsize,
                        blocks: statvfs.f_blocks,
                        bavail: statvfs.f_bavail,
                        bfree: statvfs.f_bfree,
                    })
                }
                -1 => {
                    // the filesystem wasn't found, it's a strange one, for example a
                    // docker one
                    None
                }
                _ => {
                    // unexpected
                    return Err(Error::NonZeroStavfsReturn{ code, path: mount_point });
                }
            }
        };
        let mut mount = Mount {
            id,
            parent,
            dev,
            root,
            mount_point,
            fs,
            fs_type,
            rotational: None,
            stats,
        };
        // try to determine whether it's SSD or HDD
        mount.rotational = mount.disk_name()
            .and_then(|name| sys::read_file(&format!("/sys/block/{}/queue/rotational", name)).ok())
            .and_then(|c| {
                match c.trim().as_ref() {
                "0" => Some(false),
                "1" => Some(true),
                _ => None, // should not happen today
            }});
        Ok(mount)
    }
}

pub fn read_all() -> Result<Vec<Mount>> {
    sys::read_file("/proc/self/mountinfo")?
        .trim()
        .split('\n')
        .map(str::parse)
        .inspect(|r| {
            if let Err(e) = r {
                eprintln!("Error while parsing a mount line: {}", e);
            }
        })
        .filter(Result::is_ok)
        .collect()
}
