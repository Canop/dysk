use core::str::FromStr;

/// The Units system used for sizes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Units {
    Si,     // Units according to the SI system, based on multiples of 1000
    Binary, // Old binary based units, based on multiples of 1024
    Bytes,  // Just the raw byte counts, with commas separating thousands
}

impl Default for Units {
    fn default() -> Self {
        Self::Si
    }
}
impl FromStr for Units {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_ref() {
            "si" => Ok(Self::Si),
            "binary" => Ok(Self::Binary),
            "bytes" => Ok(Self::Bytes),
            _ => Err(format!(
                "Illegal value: {:?} - valid values are 'SI', 'binary', and 'bytes'",
                value
            )),
        }
    }
}

static PREFIXES: &[char] = &['K', 'M', 'G', 'T', 'P'];

impl Units {
    pub fn fmt(
        self,
        size: u64,
    ) -> String {
        match self {
            Self::Si => file_size::fit_4(size),
            Self::Binary => {
                if size < 10_000 {
                    size.to_string()
                } else {
                    let i = size.ilog2() / 10u32;
                    let idx = i as usize - 1;
                    let size = size as f64;
                    if idx >= PREFIXES.len() {
                        "huge".to_string()
                    } else {
                        let v = size / (1024u64.pow(i) as f64);
                        if v >= 10f64 {
                            format!("{:.0}{}i", v.round(), PREFIXES[idx])
                        } else {
                            format!("{:.1}{}i", v, PREFIXES[idx])
                        }
                    }
                }
            }
            Self::Bytes => {
                let mut rev: Vec<char> = Vec::new();
                for (i, c) in size.to_string().chars().rev().enumerate() {
                    if i > 0 && i % 3 == 0 {
                        rev.push(',');
                    }
                    rev.push(c);
                }
                rev.drain(..).rev().collect()
            }
        }
    }
}

#[test]
fn test_fmt_binary() {
    fn check(
        v: u64,
        s: &str,
    ) {
        assert_eq!(&Units::Binary.fmt(v), s);
    }
    check(0, "0");
    check(1, "1");
    check(456, "456");
    check(1456, "1456");
    check(9_999, "9999");
    check(10_000, "9.8Ki");
    check(12_345, "12Ki");
    check(123_456, "121Ki");
    check(1_000_000_000, "954Mi");
    check(1_073_741_824, "1.0Gi");
    check(1_234_567_890, "1.1Gi");
}

#[test]
fn test_fmt_bytes() {
    fn check(
        v: u64,
        s: &str,
    ) {
        assert_eq!(&Units::Bytes.fmt(v), s);
    }
    check(0, "0");
    check(1, "1");
    check(456, "456");
    check(1456, "1,456");
    check(9_999, "9,999");
    check(10_000, "10,000");
    check(12_345, "12,345");
    check(123_456, "123,456");
    check(1_234_567, "1,234,567");
    check(1_000_000_000, "1,000,000,000");
    check(1_234_567_890, "1,234,567,890");
}
