use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Builtin Ty
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Builtin {
    /// natural number type
    Nat,
    ///// relative number type
    //Z,
    /// boolean type
    Bool,

    /// character type
    Char,
}

impl Pretty for Builtin {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Builtin::Nat => theme.ty_var(&"ℕ"),
            Builtin::Bool => theme.ty_var(&"𝔹"),
            Builtin::Char => theme.ty_var(&"Char"),
        }
    }
}
