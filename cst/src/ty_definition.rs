use crate::{operator, ty};
use location::{Located, Location};
use pp::prelude::*;

#[derive(Debug)]
pub struct TypeDefinition {
    pub name: ty::Variable,
    pub eq_def: operator::EqDef,
    pub ty: ty::Type,
}

impl Pretty for TypeDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let doc_ty = Doc::softline()
            .append(self.ty.pretty(theme).group())
            .nest(2);
        Doc::nil()
            .append(self.name.pretty(theme))
            .append(Doc::space())
            .append(self.eq_def.pretty(theme))
            .append(doc_ty)
    }
}

impl Located for TypeDefinition {
    fn loc(&self) -> Location {
        self.name.loc().union(self.ty.loc())
    }
}
