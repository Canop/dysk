use {
    crate::col::*,
    lfs_core::*,
    std::{
        fmt,
        str::FromStr,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColOperator {
    Lower,
    LowerOrEqual,
    Like,
    Equal,
    NotEqual,
    GreaterOrEqual,
    Greater,
}

impl ColOperator {
    pub fn eval<T: PartialOrd + PartialEq>(
        self,
        a: T,
        b: T,
    ) -> bool {
        match self {
            Self::Lower => a < b,
            Self::LowerOrEqual => a <= b,
            Self::Equal | Self::Like => a == b,
            Self::NotEqual => a != b,
            Self::GreaterOrEqual => a >= b,
            Self::Greater => a > b,
        }
    }
    pub fn eval_option<T: PartialOrd + PartialEq>(
        self,
        a: Option<T>,
        b: T,
    ) -> bool {
        match a {
            Some(a) => self.eval(a, b),
            None => false,
        }
    }
    pub fn eval_str(
        self,
        a: &str,
        b: &str,
    ) -> bool {
        match self {
            Self::Like => a.to_lowercase().contains(&b.to_lowercase()),
            _ => self.eval(a, b),
        }
    }
    pub fn eval_option_str(
        self,
        a: Option<&str>,
        b: &str,
    ) -> bool {
        match (a, self) {
            (Some(a), Self::Like) => a.to_lowercase().contains(&b.to_lowercase()),
            _ => self.eval_option(a, b),
        }
    }
}

/// A leaf in the filter expression tree, an expression which
/// may return true or false for any filesystem
#[derive(Debug, Clone, PartialEq)]
pub struct ColExpr {
    col: Col,
    operator: ColOperator,
    value: String,
}

impl ColExpr {
    #[cfg(test)]
    pub fn new<S: Into<String>>(
        col: Col,
        operator: ColOperator,
        value: S,
    ) -> Self {
        Self {
            col,
            operator,
            value: value.into(),
        }
    }
    pub fn eval(
        &self,
        mount: &Mount,
    ) -> Result<bool, EvalExprError> {
        Ok(match self.col {
            Col::Id => self.operator.eval_option(
                mount.info.id,
                self.value
                    .parse::<MountId>()
                    .map_err(|_| EvalExprError::NotAnId(self.value.to_string()))?,
            ),
            Col::Dev => self.operator.eval(
                mount.info.dev,
                self.value
                    .parse::<DeviceId>()
                    .map_err(|_| EvalExprError::NotADeviceId(self.value.to_string()))?,
            ),
            Col::Filesystem => self.operator.eval_str(&mount.info.fs, &self.value),
            Col::Label => self
                .operator
                .eval_option_str(mount.fs_label.as_deref(), &self.value),
            Col::Type => self.operator.eval_str(&mount.info.fs_type, &self.value),
            Col::Remote => self
                .operator
                .eval(mount.is_remote(), parse_bool(&self.value)?),
            Col::Disk => self
                .operator
                .eval_option_str(mount.disk.as_ref().map(|d| d.disk_type()), &self.value),
            Col::Used => self.operator.eval_option(
                mount.stats().as_ref().map(|s| s.used()),
                parse_integer(&self.value)?,
            ),
            Col::Use | Col::UsePercent => self.operator.eval_option(
                mount.stats().as_ref().map(|s| s.use_share()),
                parse_float(&self.value)?,
            ),
            Col::Free | Col::FreePercent => self.operator.eval_option(
                mount.stats().as_ref().map(|s| s.available()),
                parse_integer(&self.value)?,
            ),
            Col::Size => self.operator.eval_option(
                mount.stats().as_ref().map(|s| s.size()),
                parse_integer(&self.value)?,
            ),
            Col::InodesUsed => self.operator.eval_option(
                mount.inodes().as_ref().map(|i| i.used()),
                parse_integer(&self.value)?,
            ),
            Col::InodesUse | Col::InodesUsePercent => self.operator.eval_option(
                mount.inodes().as_ref().map(|i| i.use_share()),
                parse_float(&self.value)?,
            ),
            Col::InodesFree => self.operator.eval_option(
                mount.inodes().as_ref().map(|i| i.favail),
                parse_integer(&self.value)?,
            ),
            Col::InodesCount => self.operator.eval_option(
                mount.inodes().as_ref().map(|i| i.files),
                parse_integer(&self.value)?,
            ),
            Col::MountPoint => self
                .operator
                .eval_str(&mount.info.mount_point.to_string_lossy(), &self.value),
            Col::Uuid => self
                .operator
                .eval_option_str(mount.uuid.as_deref(), &self.value),
            Col::PartUuid => self
                .operator
                .eval_option_str(mount.part_uuid.as_deref(), &self.value),
            Col::MountOptions => self.operator.eval_str(&mount.info.options_string(), &self.value),
            Col::CompressLevel => self
                .operator
                .eval_option_str(mount.info.option_value("compress"), &self.value),
        })
    }
}

#[derive(Debug)]
pub struct ParseExprError {
    /// the string which couldn't be parsed
    pub raw: String,
    /// why
    pub message: String,
}
impl ParseExprError {
    pub fn new<R: Into<String>, M: Into<String>>(
        raw: R,
        message: M,
    ) -> Self {
        Self {
            raw: raw.into(),
            message: message.into(),
        }
    }
}
impl fmt::Display for ParseExprError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{:?} can't be parsed as an expression: {}",
            self.raw, self.message
        )
    }
}
impl std::error::Error for ParseExprError {}

impl FromStr for ColExpr {
    type Err = ParseExprError;
    fn from_str(input: &str) -> Result<Self, ParseExprError> {
        let mut chars_indices = input.char_indices();
        let mut op_idx = 0;
        for (idx, c) in &mut chars_indices {
            if c == '<' || c == '>' || c == '=' {
                op_idx = idx;
                break;
            }
        }
        if op_idx == 0 {
            return Err(ParseExprError::new(
                input,
                "Invalid expression; expected <column><operator><value>",
            ));
        }
        let mut val_idx = op_idx + 1;
        for (idx, c) in &mut chars_indices {
            if c != '<' && c != '>' && c != '=' {
                val_idx = idx;
                break;
            }
        }
        if val_idx == input.len() {
            return Err(ParseExprError::new(input, "no value"));
        }
        let col = &input[..op_idx];
        let col = col
            .parse()
            .map_err(|e: ParseColError| ParseExprError::new(input, e.to_string()))?;
        let operator = match &input[op_idx..val_idx] {
            "<" => ColOperator::Lower,
            "<=" => ColOperator::LowerOrEqual,
            "=" => ColOperator::Like,
            "==" => ColOperator::Equal,
            "<>" => ColOperator::NotEqual,
            ">=" => ColOperator::GreaterOrEqual,
            ">" => ColOperator::Greater,
            op => {
                return Err(ParseExprError::new(
                    input,
                    format!("unknown operator: {:?}", op),
                ));
            }
        };
        let value = &input[val_idx..];
        let value = value.into();
        Ok(Self {
            col,
            operator,
            value,
        })
    }
}

#[test]
fn test_col_filter_parsing() {
    assert_eq!(
        "remote=false".parse::<ColExpr>().unwrap(),
        ColExpr::new(Col::Remote, ColOperator::Like, "false"),
    );
    assert_eq!(
        "size<32G".parse::<ColExpr>().unwrap(),
        ColExpr::new(Col::Size, ColOperator::Lower, "32G"),
    );
}

#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum EvalExprError {
    NotANumber(String),
    NotAnId(String),
    NotADeviceId(String),
    NotABool(String),
}
impl EvalExprError {}
impl fmt::Display for EvalExprError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NotANumber(s) => {
                write!(f, "{:?} can't be evaluated as a number", &s)
            }
            Self::NotAnId(s) => {
                write!(f, "{:?} can't be evaluated as an id", &s)
            }
            Self::NotADeviceId(s) => {
                write!(f, "{:?} can't be evaluated as a device id", &s)
            }
            Self::NotABool(s) => {
                write!(f, "{:?} can't be evaluated as a boolean", &s)
            }
        }
    }
}
impl std::error::Error for EvalExprError {}

fn parse_bool(input: &str) -> Result<bool, EvalExprError> {
    let s = input.to_lowercase();
    match s.as_ref() {
        "x" | "t" | "true" | "1" | "y" | "yes" => Ok(true),
        "f" | "false" | "0" | "n" | "no" => Ok(false),
        _ => Err(EvalExprError::NotABool(input.to_string())),
    }
}

/// Parse numbers like "1234", "32G", "4kB", "54Gib", "1.2M"
fn parse_integer(input: &str) -> Result<u64, EvalExprError> {
    let s = input.to_lowercase();
    let s = s.trim_end_matches('b');
    let (s, binary) = match s.strip_suffix('i') {
        Some(s) => (s, true),
        None => (s, false),
    };
    let cut = s.find(|c: char| !(c.is_ascii_digit() || c == '.'));
    let (digits, factor): (&str, u64) = match cut {
        Some(idx) => (
            &s[..idx],
            match (&s[idx..], binary) {
                ("k", false) => 1000,
                ("k", true) => 1024,
                ("m", false) => 1000 * 1000,
                ("m", true) => 1024 * 1024,
                ("g", false) => 1000 * 1000 * 1000,
                ("g", true) => 1024 * 1024 * 1024,
                ("t", false) => 1000 * 1000 * 1000 * 1000,
                ("t", true) => 1024 * 1024 * 1024 * 1024,
                _ => {
                    // it's not a number
                    return Err(EvalExprError::NotANumber(input.to_string()));
                }
            },
        ),
        None => (s, 1),
    };
    match digits.parse::<f64>() {
        Ok(n) => Ok((n * factor as f64).ceil() as u64),
        _ => Err(EvalExprError::NotANumber(input.to_string())),
    }
}

#[test]
fn test_parse_integer() {
    assert_eq!(parse_integer("33"), Ok(33));
    assert_eq!(parse_integer("55G"), Ok(55_000_000_000));
    assert_eq!(parse_integer("1.23kiB"), Ok(1260));
}

/// parse numbers like "0.25", "50%"
fn parse_float(input: &str) -> Result<f64, EvalExprError> {
    let s = input.to_lowercase();
    let (s, percent) = match s.strip_suffix('%') {
        Some(s) => (s, true),
        None => (s.as_str(), false),
    };
    let mut n = s
        .parse::<f64>()
        .map_err(|_| EvalExprError::NotANumber(input.to_string()))?;
    if percent {
        n /= 100.0;
    }
    Ok(n)
}

#[test]
fn test_parse_float() {
    assert_eq!(parse_float("50%").unwrap().to_string(), "0.5".to_string());
}
