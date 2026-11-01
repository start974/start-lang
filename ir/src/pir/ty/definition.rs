use crate::location::{Span, Spanned};
use crate::pir::{Variable, operator, ty};
use crate::token::Token;
use pp::pretty::*;

#[derive(Debug)]
pub struct TypeDefinition {
    pub name: Variable,
    pub eq_def: Token<operator::EqDef>,
    pub ty: ty::Type,
}

impl Pretty for TypeDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let doc_ty = Doc::softline()
            .append(self.ty.pretty(theme).group())
            .nest(2);
        Doc::nil()
            .append(self.name.pretty_with(theme, theme.ty_var(&self.name.value)))
            .append(Doc::space())
            .append(self.eq_def.pretty(theme))
            .append(doc_ty)
    }
}

impl Spanned for TypeDefinition {
    fn span(&self) -> Span {
        self.name.span().union(self.ty.span())
    }
}
