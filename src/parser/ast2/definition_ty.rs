use super::{Identifier, Ty};
pub use crate::location2::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct TyDefinition<Path> {
    name: Identifier<Path>,
    ty: Ty<Path>,
}

impl<Path> TyDefinition<Path> {
    /// make a new type definition
    pub fn new(name: Identifier<Path>, ty: Ty<Path>) -> Self {
        Self { name, ty }
    }

    /// get name of type
    pub fn name(&self) -> &Identifier<Path> {
        &self.name
    }

    /// get type of type definition
    pub fn ty(&self) -> &Ty<Path> {
        &self.ty
    }
}

impl<Path> Located<Path> for TyDefinition<Path> {
    /// location is at name of definition
    fn loc(&self) -> &Location<Path> {
        self.name.loc()
    }
}

impl<Path> Pretty for TyDefinition<Path> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::group(
            Doc::nil()
                .append(theme.kw_type())
                .append(Doc::space())
                .append(theme.def_var(self.name.name()))
                .append(Doc::space())
                .append(theme.op_eq_def())
                .append(Doc::line())
                .append(Doc::group(self.ty.pretty(theme))),
        )
    }
}
