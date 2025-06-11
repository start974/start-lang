use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::identifier::Identifier;

// ==========================================================================
// Type
// ==========================================================================
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
            Ty::Var(ident) => theme.ty_var(ident),
        }
    }
}

// ==========================================================================
// Definition Type
// ==========================================================================
pub struct TyDefinition {
    name: Identifier,
    ty: Ty,
}

impl TyDefinition {
    /// make a new type definition
    pub fn new(name: Identifier, ty: Ty) -> Self {
        Self { name, ty }
    }

    /// get name of type
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    /// get type of type definition
    pub fn ty(&self) -> &Ty {
        &self.ty
    }
}

impl Located for TyDefinition {
    /// location is at name of definition
    fn loc(&self) -> &Location {
        self.name.loc()
    }
}

impl Pretty for TyDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::group(
            Doc::nil()
                .append(theme.kw_type())
                .append(Doc::space())
                .append(theme.def_var(&self.name))
                .append(Doc::space())
                .append(theme.op_eq_def())
                .append(Doc::line())
                .append(Doc::group(self.ty.pretty(theme))),
        )
    }
}
