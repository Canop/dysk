use {
    std::time::Duration,
    std::str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timeout(Option<Duration>);

impl Timeout {
    pub fn as_duration(&self) -> Option<Duration> {
        self.0
    }
    fn try_read(s: &str) -> Option<Self> {
        if s == "none" || s == "no" {
            return Some(Self(None));
        }
        if let Some(n) = s.strip_suffix("ms") {
            if let Ok(n) = n.parse::<u64>() {
                return Some(Self(Some(Duration::from_millis(n))));
            }
        } else if let Some(n) = s.strip_suffix("s") {
            if let Ok(n) = n.parse::<u64>() {
                return Some(Self(Some(Duration::from_secs(n))));
            }
        }
        None
    }
}

impl FromStr for Timeout {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_read(s)
            .ok_or(r#"Invalid timeout, expected "none" or <number>[s|ms]"#)
    }
}

