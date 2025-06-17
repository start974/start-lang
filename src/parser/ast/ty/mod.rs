use super::identifier::Identifier;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

mod definition;

pub use definition::Definition as TypeDefinition;

// ==========================================================================
// Type
// ==========================================================================
/// constant types
#[derive(Debug)]
pub enum Type {
    Var(Identifier),
}

impl From<Identifier> for Type {
    fn from(ident: Identifier) -> Self {
        Type::Var(ident)
    }
}

impl Located for Type {
    fn loc(&self) -> &Location {
        match self {
            Type::Var(ident) => ident.loc(),
        }
    }
}

impl Pretty for Type {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Type::Var(ident) => theme.ty_var(ident),
        }
    }
}
