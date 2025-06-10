use crate::location2::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::identifier::Identifier;

/// constant types
pub enum Ty {
    Var(Identifier),
}

impl Ty {
    /// make variable type
    pub fn var(ident: Identifier) -> Self {
        Self::Var(ident)
    }
}

impl Located for Ty {
    fn loc(&self) -> &Location {
        match self {
            Ty::Var(ident) => ident.loc(),
        }
    }
}

impl Pretty for Ty {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Ty::Var(ident) => theme.ty_var(ident.name()),
        }
    }
}
