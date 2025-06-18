use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Builtin Ty
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Builtin {
    /// natural number type
    N,
    ///// relative number type
    //Z,
    ///// boolean type
    //B,
}

impl Pretty for Builtin {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Builtin::N => theme.ty_var(&"ℕ"),
            //Kind::Z => theme.ty_var(&"ℤ"),
            //Kind::B => theme.ty_var(&"𝔹"),
        }
    }
}
