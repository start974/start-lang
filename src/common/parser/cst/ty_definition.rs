use super::{operator, ty};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

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
