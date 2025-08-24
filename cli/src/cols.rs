use {
    crate::col::*,
    std::str::FromStr,
};

/// Sequence of columns, ordered
#[derive(Debug, Clone, PartialEq)]
pub struct Cols(pub Vec<Col>);

impl Default for Cols {
    fn default() -> Self {
        Self(DEFAULT_COLS.to_vec())
    }
}

impl Cols {
    #[cfg(test)]
    pub fn new<V: Into<Vec<Col>>>(v: V) -> Self {
        Self(v.into())
    }
    pub fn empty() -> Self {
        Self(Vec::new())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn contains(
        &self,
        tbl: Col,
    ) -> bool {
        self.0.contains(&tbl)
    }
    pub fn remove(
        &mut self,
        removed: Col,
    ) {
        self.0.retain(|&f| f != removed);
    }
    /// Add a col, preventing duplicates
    /// (may be used when the col is present to reorder)
    pub fn add(
        &mut self,
        added: Col,
    ) {
        self.remove(added);
        self.0.push(added);
    }
    /// Add the columns of the set, except when they're
    /// already present
    ///
    /// This makes it possible to add a set while keeping
    /// the order of the previous columns, for example
    /// `dysk -c disk+`
    pub fn add_set(
        &mut self,
        col_set: &[Col],
    ) {
        if self.0 == ALL_COLS {
            for &col in col_set {
                self.add(col);
            }
        } else {
            for &col in col_set {
                if !self.contains(col) {
                    self.add(col);
                }
            }
        }
    }
    pub fn remove_set(
        &mut self,
        col_set: &[Col],
    ) {
        for &col in col_set {
            self.remove(col);
        }
    }
    pub fn cols(&self) -> &[Col] {
        &self.0
    }
}

impl FromStr for Cols {
    type Err = ParseColError;
    fn from_str(value: &str) -> Result<Self, ParseColError> {
        let value = value.trim();
        let mut tokens: Vec<String> = Vec::new();
        let mut must_create = true;
        for c in value.chars() {
            if c.is_alphabetic() || c == '_' {
                if must_create {
                    tokens.push(c.into());
                    must_create = false;
                } else {
                    let len = tokens.len();
                    tokens[len - 1].push(c);
                }
            } else {
                tokens.push(c.into());
                must_create = true;
            }
        }
        let mut cols = if let Some(first_token) = tokens.first() {
            if first_token == "+" || first_token == "-" {
                // if it starts with an addition or removal, the
                // default set is implied
                Cols::default()
            } else {
                Cols::empty()
            }
        } else {
            return Ok(Self::default());
        };
        let mut negative = false;
        for token in &tokens {
            match token.as_ref() {
                "-" => {
                    negative = true;
                }
                "+" | "," | " " => {}
                "all" => {
                    if negative {
                        cols = Cols::empty();
                        negative = false;
                    } else {
                        // if we add all to something, it means the already
                        // present one are meant to be first
                        for &col in ALL_COLS {
                            if !cols.contains(col) {
                                cols.add(col);
                            }
                        }
                    }
                }
                "default" => {
                    if negative {
                        cols.remove_set(DEFAULT_COLS);
                        negative = false;
                    } else {
                        cols.add_set(DEFAULT_COLS);
                    }
                }
                _ => {
                    let col: Col = token.parse()?;
                    if negative {
                        cols.remove(col);
                        negative = false;
                    } else {
                        cols.add(col);
                    }
                }
            }
        }
        match tokens.last().map(|s| s.as_ref()) {
            Some("-") => {
                cols.remove_set(DEFAULT_COLS);
            }
            Some("+") => {
                cols.add_set(DEFAULT_COLS);
            }
            _ => {}
        }
        Ok(cols)
    }
}

#[cfg(test)]
mod cols_parsing {
    use super::{
        Col::*,
        *,
    };

    fn check<V: Into<Vec<Col>>>(
        s: &str,
        v: V,
    ) {
        println!("cols definition: {s:?}");
        let from_str: Cols = s.parse().unwrap();
        let from_vec: Cols = Cols::new(v);
        assert_eq!(from_str, from_vec);
    }

    #[test]
    fn bad_cols() {
        assert_eq!(
            "nothing".parse::<Cols>().unwrap_err().to_string(),
            r#""nothing" can't be parsed as a column; use 'dysk --list-cols' to see all column names"#,
        );
    }

    #[test]
    fn explicit_cols() {
        check("dev", vec![Dev]);
        check("dev,free,used", vec![Dev, Free, Used]);
        check("dev+free + used", vec![Dev, Free, Used]);
        check("  dev   free used ", vec![Dev, Free, Used]);
        check("all", ALL_COLS);
    }

    #[test]
    fn algebraic_cols() {
        check(
            "all - dev -inodes + label",
            vec![
                Id,
                Filesystem,
                Type,
                Remote,
                Disk,
                Used,
                Use,
                UsePercent,
                Free,
                FreePercent,
                Size,
                InodesUsed,
                InodesUsePercent,
                InodesFree,
                InodesCount,
                MountPoint,
                Uuid,
                PartUuid,
                Label,
            ],
        );
        check("dev + dev +disk - use + size", vec![Dev, Disk, Size]);
        check(
            "all-default+use",
            vec![
                Id,
                Dev,
                Label,
                Remote,
                UsePercent,
                FreePercent,
                InodesUsed,
                InodesUse,
                InodesUsePercent,
                InodesFree,
                InodesCount,
                Uuid,
                PartUuid,
                Use,
            ],
        );
        check(
            "all+default", // special: all but default at the end
            vec![
                Id,
                Dev,
                Label,
                Remote,
                UsePercent,
                FreePercent,
                InodesUsed,
                InodesUse,
                InodesUsePercent,
                InodesFree,
                InodesCount,
                Uuid,
                PartUuid,
                Filesystem,
                Type,
                Disk,
                Used,
                Use,
                Free,
                Size,
                MountPoint,
            ],
        );
        check(
            "fs dev all", // we want all column but fs and dev at the start
            vec![
                Filesystem,
                Dev,
                Id,
                Label,
                Type,
                Remote,
                Disk,
                Used,
                Use,
                UsePercent,
                Free,
                FreePercent,
                Size,
                InodesUsed,
                InodesUse,
                InodesUsePercent,
                InodesFree,
                InodesCount,
                MountPoint,
                Uuid,
                PartUuid,
            ],
        );
        check(
            "fs dev all -id-disk",
            vec![
                Filesystem,
                Dev,
                Label,
                Type,
                Remote,
                Used,
                Use,
                UsePercent,
                Free,
                FreePercent,
                Size,
                InodesUsed,
                InodesUse,
                InodesUsePercent,
                InodesFree,
                InodesCount,
                MountPoint,
                Uuid,
                PartUuid,
            ],
        );
    }

    #[test]
    fn cols_from_default() {
        check("", DEFAULT_COLS);
        check(
            "-dev", // no impact as dev isn't in defaults
            DEFAULT_COLS,
        );
        check("default", DEFAULT_COLS);
        check(
            "-default", // not really useful
            vec![],
        );
        check(
            "default-dev", // no impact as dev isn't in defaults
            DEFAULT_COLS,
        );
        check(
            "+dev",
            vec![
                Filesystem, Type, Disk, Used, Use, Free, Size, MountPoint, Dev,
            ],
        );
        check(
            "dev+",
            vec![
                Dev, Filesystem, Type, Disk, Used, Use, Free, Size, MountPoint,
            ],
        );
        check(
            "all-",
            vec![
                Id,
                Dev,
                Label,
                Remote,
                UsePercent,
                FreePercent,
                InodesUsed,
                InodesUse,
                InodesUsePercent,
                InodesFree,
                InodesCount,
                Uuid,
                PartUuid,
            ],
        );
        check(
            "-size+inodes_free+",
            vec![
                Filesystem, Type, Disk, Used, Use, Free, MountPoint, InodesFree, Size,
            ],
        );
        check(
            "+dev-size+inodes_use",
            vec![
                Filesystem, Type, Disk, Used, Use, Free, MountPoint, Dev, InodesUse,
            ],
        );
        check(
            "-use-type",
            vec![Filesystem, Disk, Used, Free, Size, MountPoint],
        );
        check(
            "default+dev",
            vec![
                Filesystem, Type, Disk, Used, Use, Free, Size, MountPoint, Dev,
            ],
        );
        check(
            "default,size+use", // just reordering
            vec![Filesystem, Type, Disk, Used, Free, MountPoint, Size, Use],
        );
        check(
            "dev default",
            vec![
                Dev, Filesystem, Type, Disk, Used, Use, Free, Size, MountPoint,
            ],
        );
        check(
            "size dev default -disk",
            vec![Size, Dev, Filesystem, Type, Used, Use, Free, MountPoint],
        );
        check(
            "default-fs+inodes",
            vec![Type, Disk, Used, Use, Free, Size, MountPoint, InodesUse],
        );
        check(
            "+inodes_used+inodes_free",
            vec![
                Filesystem, Type, Disk, Used, Use, Free, Size, MountPoint, InodesUsed, InodesFree,
            ],
        );
    }
}
