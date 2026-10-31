use ariadne::Cache;
use ir::location::SourceId;
use pp::theme::Theme;

use crate::Error;

#[derive(Debug)]
pub struct Errors {
    source_id: SourceId,
    errs: Vec<Error>,
}

impl Errors {
    pub fn with_source_id(mut self, source_id: SourceId) -> Self {
        self.source_id = source_id;
        self
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

    /// number of errors
    pub fn len(&self) -> usize {
        self.errs.len()
    }

    /// no error
    pub fn is_empty(&self) -> bool {
        self.errs.is_empty()
    }

    /// exit code: the code of the only error, `1` for several, `0` for none
    pub fn code(&self) -> i32 {
        match self.errs.as_slice() {
            [] => 0,
            [err] => err.code(),
            _ => 1,
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
    use ir::location::Span;

    #[test]
    fn singleton() {
        let err = Error::new(1001, Message::text("Header")).with_span(Span::new(0, 1));
        let errs = Errors::from(err);

        assert_eq!(errs.code(), 1001);
        assert_eq!(errs.len(), 1);
    }

    #[test]
    fn append() {
        let err1 = Error::new(1001, Message::text("Header1")).with_span(Span::new(0, 1));
        let err2 = Error::new(1002, Message::text("Header2")).with_span(Span::new(2, 3));
        let errs = Errors::from(err1).append(err2);
        assert_eq!(errs.code(), 1);
        assert_eq!(errs.len(), 2);
    }

    #[test]
    fn combine() {
        let err1 = Error::new(1001, Message::text("Header1")).with_span(Span::new(0, 1));
        let err2 = Error::new(1002, Message::text("Header2")).with_span(Span::new(2, 3));
        let errs1 = Errors::from(err1);
        let errs2 = Errors::from(err2);
        let combined = errs1.combine(errs2);

        assert_eq!(combined.code(), 1);
        assert_eq!(combined.len(), 2);
    }

    #[test]
    fn source_id() {
        let err = Error::new(1001, Message::text("Header")).with_span(Span::new(0, 1));
        let errs = Errors::from(err);
        assert_eq!(errs.source_id, SourceId::Unknown);
        let errs = errs.with_source_id(SourceId::Repl);
        assert_eq!(errs.source_id, SourceId::Repl);
    }

    #[test]
    fn from_iter() {
        let err1 = Error::new(1001, Message::text("Header1")).with_span(Span::new(0, 1));
        let err2 = Error::new(1002, Message::text("Header2")).with_span(Span::new(2, 3));
        let errs: Errors = vec![err1, err2].into_iter().collect();

        assert_eq!(errs.code(), 1);
        assert_eq!(errs.len(), 2);

        let mut iter = errs.into_iter();
        assert_eq!(iter.next().unwrap().code(), 1001);
        assert_eq!(iter.next().unwrap().code(), 1002);
    }
}
