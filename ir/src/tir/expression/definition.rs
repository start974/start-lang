use crate::tir::{Expression, Pattern, Type, Typed};
use location::{Span, Spanned};
use pp::pretty::*;

// ==========================================================================
// Expression Definition
// ==========================================================================
pub struct ExpressionDefinition {
    /// defined pattern
    pattern: Pattern,

    /// body of definition
    body: Expression,
}

impl ExpressionDefinition {
    /// Create a new expression definition
    pub fn new(pattern: Pattern, body: Expression) -> Self {
        Self { pattern, body }
    }

    /// get pattern of definition
    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Get the body of the expression definition
    pub fn body(&self) -> &Expression {
        &self.body
    }
}

impl Typed for ExpressionDefinition {
    fn ty(&self) -> &Type {
        self.body.ty()
    }
}

impl Pretty for ExpressionDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(theme.keyword(&"Definition"))
            .append(Doc::space())
            .append(self.pattern.pretty(theme))
            .append(Doc::group(
                Doc::nil().append(
                    Doc::line()
                        .append(theme.operator(&":"))
                        .append(Doc::space())
                        .append(self.ty().pretty(theme))
                        .group()
                        .nest(4),
                ),
            ))
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
