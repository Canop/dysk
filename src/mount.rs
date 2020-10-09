
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

/// A mount point
#[derive(Debug)]
pub struct Mount {
    pub id: MountId,
    pub parent: MountId,
    pub dev: DeviceId,
    pub root: PathBuf,
    pub mount_point: PathBuf,
    pub fs_type: String,
    pub bsize: u64,
    pub blocks: u64,
    pub bavail: u64,
    pub bfree: u64,
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
        // we get the free/total space info in libc::statvfs
        let c_mount_point = CString::new(mount_point.as_os_str().as_bytes()).unwrap();
        let statvfs = unsafe {
            let mut stat = mem::MaybeUninit::<libc::statvfs>::uninit();
            let res = libc::statvfs(c_mount_point.as_ptr(), stat.as_mut_ptr());
            if res != 0 {
                return Err(Error::NonZeroLibcReturn(res));
            }
            stat.assume_init()
        };
        Ok(Mount {
            id,
            parent,
            dev,
            root,
            mount_point,
            fs_type,
            bsize: statvfs.f_bsize,
            blocks: statvfs.f_blocks,
            bavail: statvfs.f_bavail,
            bfree: statvfs.f_bfree,
        })
    }
}

pub fn read_all() -> Result<Vec<Mount>> {
    sys::read_file("/proc/self/mountinfo")?
        .trim()
        .split('\n')
        .map(str::parse)
        .collect()
}
