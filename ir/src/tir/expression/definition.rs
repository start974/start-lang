use crate::location::{Span, Spanned};
use crate::tir::{Expression, Pattern};
use pp::pretty::*;

// ============================================================================
// Expression Definition
// ============================================================================

/// `pattern := body`; the type of the definition is the type of `body`
/// (restricted to the declared type, if any)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionDefinition {
    pub pattern: Pattern,
    pub body: Expression,
}

impl Spanned for ExpressionDefinition {
    fn span(&self) -> Span {
        self.pattern.span().union(self.body.span())
    }
}

impl Pretty for ExpressionDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.pattern
            .pretty(theme)
            .append(
                Doc::line()
                    .append(theme.operator(&":"))
                    .append(Doc::space())
                    .append(self.body.ty.pretty(theme))
                    .nest(4)
                    .group(),
            )
            .append(Doc::space())
            .append(theme.operator(&":="))
            .append(Doc::line().append(self.body.pretty(theme).group()).nest(2))
    }
}
