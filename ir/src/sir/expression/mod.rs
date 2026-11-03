use crate::location::{Span, Spanned};
use crate::sir::{Constant, Identifier, Node, Type};
use pp::pretty::*;

pub mod constant;
pub mod definition;
pub mod pattern;

// ============================================================================
// Expression
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    /// use of a definition, resolved by the lowering
    Variable(Node<Identifier>),

    /// constant
    Constant(Constant),

    /// expression restricted to a type: `e : ty`
    Typed { expr: Box<Expression>, ty: Type },
}

/// expression, with the span of the source it comes from
/// (parentheses included)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub span: Span,
}

impl Pretty for Expression {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self.kind {
            ExpressionKind::Variable(var) => theme.expr_var(&var.value),
            ExpressionKind::Constant(c) => c.pretty(theme),
            // always in parentheses: no precedence to know when they can go
            ExpressionKind::Typed { expr, ty } => theme
                .operator(&"(")
                .append(expr.pretty(theme))
                .append(Doc::space())
                .append(theme.operator(&":"))
                .append(Doc::space())
                .append(ty.pretty(theme))
                .append(theme.operator(&")")),
        }
    }
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        self.span
    }
}
