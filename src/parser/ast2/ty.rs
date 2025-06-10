use crate::location2::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::identifier::Identifier;

/// constant types
pub enum Ty<Path> {
    Var(Identifier<Path>),
}

impl<Path> Ty<Path> {
    /// make variable type
    pub fn var(ident: Identifier<Path>) -> Self {
        Self::Var(ident)
    }
}

impl<Path> Located<Path> for Ty<Path> {
    fn loc(&self) -> &Location<Path> {
        match self {
            Ty::Var(ident) => ident.loc(),
        }
    }
}

impl<Path> Pretty for Ty<Path> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Ty::Var(ident) => theme.ty_var(ident.name()),
        }
    }
}
