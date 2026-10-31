//! Errors found while building `tir`, as data: the diagnostics (code,
//! message) are made by `driver::typing`, so `ir` does not depend on
//! `errors`.

use crate::location::{Span, Spanned};
use crate::tir::env::IdentifierKind;
use crate::tir::{Identifier, Type};

#[derive(Debug, Clone)]
pub enum Error {
    /// no definition of this kind for `identifier`
    VariableNotFound {
        identifier: Identifier,
        kind: IdentifierKind,
        span: Span,
    },
    /// the type `found` was restricted to the incompatible type `expected`
    UnexpectedType {
        expected: Box<Type>,
        found: Box<Type>,
        span: Span,
    },
}

impl Spanned for Error {
    fn span(&self) -> Span {
        match self {
            Error::VariableNotFound { span, .. } | Error::UnexpectedType { span, .. } => *span,
        }
    }
}
