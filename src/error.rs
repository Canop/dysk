
/// lfs error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Unexpected format")]
    UnexpectedFormat,
    #[error("parse int error: {0}")]
    NotAnInt(#[from] std::num::ParseIntError),
    #[error("non zero libc return code: {0}")]
    NonZeroLibcReturn(i32),
}

pub type Result<T> = std::result::Result<T, Error>;
