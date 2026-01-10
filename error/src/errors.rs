use ariadne::Cache;
use location::SourceId;
use pp::theme::Theme;

use crate::Error;

pub struct Errors {
    errs: Vec<Error>,
}

impl Errors {
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
            err.eprint(theme, cache);
        }
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
        Self { errs: vec![e] }
    }
}
