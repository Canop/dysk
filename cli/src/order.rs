use std::{
    fmt,
    str::FromStr,
};

/// one of the two sorting directions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug)]
pub struct ParseOrderError {
    /// the string which couldn't be parsed
    pub raw: String,
}
impl ParseOrderError {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self { raw: s.into() }
    }
}
impl fmt::Display for ParseOrderError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{:?} can't be parsed as a sort order. Use 'asc' or 'desc' (or nothing)",
            self.raw
        )
    }
}
impl std::error::Error for ParseOrderError {}

impl FromStr for Order {
    type Err = ParseOrderError;
    fn from_str(s: &str) -> Result<Self, ParseOrderError> {
        let s = s.to_lowercase();
        match s.as_ref() {
            "a" | "asc" => Ok(Self::Asc),
            "d" | "desc" => Ok(Self::Desc),
            _ => Err(ParseOrderError::new(s)),
        }
    }
}
