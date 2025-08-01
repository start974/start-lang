use super::super::ty::Type;
use super::TypeVariable;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Definition Type
// ==========================================================================
#[derive(Debug)]
pub struct Definition {
    name: TypeVariable,
    ty: Type,
}

impl Definition {
    /// make a new type definition
    pub fn new(name: TypeVariable, ty: Type) -> Self {
        Self { name, ty }
    }

    /// get name of type
    pub fn name(&self) -> &TypeVariable {
        &self.name
    }

    /// get type of type definition
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

impl Located for Definition {
    /// location is at name of definition
    fn loc(&self) -> &Location {
        self.name.loc()
    }
}

impl Pretty for Definition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::group(
            Doc::nil()
                .append(theme.def_var(&self.name))
                .append(Doc::space())
                .append(theme.operator(&":="))
                .append(Doc::line().append(self.ty.pretty(theme).group()).nest(2)),
        )
    }
}
