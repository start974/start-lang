use crate::utils::location::{Located, LocatedSet, Location, UNKNOWN_LOCATION};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Builtin Ty
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Builtin {
    /// natural number type
    N,
    /// relative number type
    Z,
    /// boolean type
    B,
}

impl Located for Builtin {
    fn loc(&self) -> &Location {
        &UNKNOWN_LOCATION
    }
}

impl Pretty for Builtin {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Builtin::N => theme.ty_var(&"ℕ"),
            Builtin::Z => theme.ty_var(&"ℤ"),
            Builtin::B => theme.ty_var(&"𝔹"),
        }
    }
}
