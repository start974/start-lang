use super::super::identifier::Identifier;
use super::super::ty::Type;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Definition Type
// ==========================================================================
#[derive(Debug)]
pub struct Definition {
    name: Identifier,
    ty: Type,
}

impl Definition {
    /// make a new type definition
    pub fn new(name: Identifier, ty: Type) -> Self {
        Self { name, ty }
    }

    /// get name of type
    pub fn name(&self) -> &Identifier {
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
                .append(theme.op_eq_def())
                .append(Doc::line())
                .append(Doc::group(self.ty.pretty(theme))),
        )
    }
}
