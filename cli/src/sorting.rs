use {
    crate::{
        col::Col,
        order::Order,
    },
    lfs_core::Mount,
    std::{
        error,
        fmt,
        str::FromStr,
    },
};

/// Sorting directive: the column and the order (asc or desc)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sorting {
    col: Col,
    order: Order,
}

impl Default for Sorting {
    fn default() -> Self {
        let col = Col::default_sort_col();
        let order = col.default_sort_order();
        Self { col, order }
    }
}

impl Sorting {
    pub fn sort(
        self,
        mounts: &mut [Mount],
    ) {
        let comparator = self.col.comparator();
        mounts.sort_by(comparator);
        if self.order == Order::Desc {
            mounts.reverse();
        }
    }
}

#[derive(Debug)]
pub struct ParseSortingError {
    raw: String,
    reason: String,
}
impl ParseSortingError {
    pub fn new<S: Into<String>, E: ToString>(
        raw: S,
        reason: E,
    ) -> Self {
        Self {
            raw: raw.into(),
            reason: reason.to_string(),
        }
    }
}
impl fmt::Display for ParseSortingError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{:?} can't be parsed as a sort expression because {}",
            self.raw, self.reason
        )
    }
}
impl error::Error for ParseSortingError {}

impl FromStr for Sorting {
    type Err = ParseSortingError;
    fn from_str(s: &str) -> Result<Self, ParseSortingError> {
        let cut_idx_len = s
            .char_indices()
            .find(|(_idx, c)| c.is_whitespace() || *c == '-')
            .map(|(idx, c)| (idx, c.len_utf8()));
        let (s_col, s_order) = match cut_idx_len {
            Some((idx, len)) => (&s[..idx], Some(&s[idx + len..])),
            None => (s, None),
        };
        let col: Col = s_col
            .parse()
            .map_err(|pce| ParseSortingError::new(s, Box::new(pce)))?;
        let order = match s_order {
            Some(s_order) => s_order
                .parse()
                .map_err(|poe| ParseSortingError::new(s, Box::new(poe)))?,
            None => col.default_sort_order(),
        };
        Ok(Self { col, order })
    }
}
