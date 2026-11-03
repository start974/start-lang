use crate::location::{Span, Spanned};
use crate::sir::{Expression, Pattern, Type};
use pp::pretty::*;

// ============================================================================
// Expression Definition
// ============================================================================

/// `pattern [: ty] := body`, the definition `id`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionDefinition {
    pub pattern: Pattern,
    pub ty: Option<Type>,
    pub body: Expression,
}

impl Pretty for ExpressionDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let ty = match &self.ty {
            Some(ty) => Doc::line()
                .append(theme.operator(&":"))
                .append(Doc::space())
                .append(ty.pretty(theme))
                .nest(4)
                .group(),
            None => Doc::nil(),
        };
        self.pattern
            .pretty(theme)
            .append(ty)
            .append(Doc::space())
            .append(theme.operator(&":="))
            .append(Doc::line().append(self.body.pretty(theme).group()).nest(2))
    }
}

impl Spanned for ExpressionDefinition {
    fn span(&self) -> Span {
        self.pattern.span().union(self.body.span())
    }
}
