use ariadne::Cache;
use location::SourceId;
use pp::theme::Theme;

use crate::Error;

pub struct Errors {
    source_id: SourceId,
    errs: Vec<Error>,
}

impl Errors {
    pub fn with_source_id(source_id: SourceId) -> Self {
        Self {
            source_id,
            errs: Vec::new(),
        }
    }

    /// add error
    pub fn append(mut self, e: Error) -> Self {
        self.errs.push(e);
        self
    }

    /// combine two errors
    pub fn combine(mut self, mut other: Errors) -> Self {
        self.errs.append(&mut other.errs);
        self
    }

    /// print all errors on stderr
    pub fn eprint(&self, theme: &Theme, cache: &mut impl Cache<SourceId>) {
        for err in &self.errs {
            err.eprint(&self.source_id, theme, cache);
        }
    }

    /// lenght of errors
    pub fn lenght(&self) -> usize {
        self.errs.len()
    }

    /// get code
    pub fn code(&self) -> i32 {
        if self.errs.len() > 1 {
            1
        } else {
            self.errs.first().map(|e| e.code()).unwrap()
        }
    }
}

impl From<Error> for Errors {
    fn from(e: Error) -> Self {
        Self {
            source_id: SourceId::Unknown,
            errs: vec![e],
        }
    }
}

impl FromIterator<Error> for Errors {
    fn from_iter<T: IntoIterator<Item = Error>>(iter: T) -> Self {
        Self {
            source_id: SourceId::Unknown,
            errs: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Errors {
    type Item = Error;
    type IntoIter = std::vec::IntoIter<Error>;

    fn into_iter(self) -> Self::IntoIter {
        self.errs.into_iter()
    }
}

// ============================================================================
// Test
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Error, Message};
    use location::Span;

    #[test]
    fn singleton() {
        let err = Error::new(1001, Message::text("Header")).with_span(Span::new(0, 1));
        let errs = Errors::from(err);

        assert_eq!(errs.code(), 1001);
        assert_eq!(errs.lenght(), 1);
    }

    #[test]
    fn append() {
        let err1 = Error::new(1001, Message::text("Header1")).with_span(Span::new(0, 1));
        let err2 = Error::new(1002, Message::text("Header2")).with_span(Span::new(2, 3));
        let errs = Errors::from(err1).append(err2);
        assert_eq!(errs.code(), 1);
        assert_eq!(errs.lenght(), 2);
    }

    #[test]
    fn combine() {
        let err1 = Error::new(1001, Message::text("Header1")).with_span(Span::new(0, 1));
        let err2 = Error::new(1002, Message::text("Header2")).with_span(Span::new(2, 3));
        let errs1 = Errors::from(err1);
        let errs2 = Errors::from(err2);
        let combined = errs1.combine(errs2);

        assert_eq!(combined.code(), 1);
        assert_eq!(combined.lenght(), 2);
    }
}
