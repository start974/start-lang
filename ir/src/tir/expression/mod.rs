use crate::location::{Span, Spanned};
use crate::tir::{Constant, Identifier, Node, Type};
use pp::pretty::*;

pub mod constant;
pub mod definition;
pub mod pattern;

// ============================================================================
// Expression
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    /// use of a definition
    Variable(Node<Identifier>),

    /// constant
    Constant(Constant),
}

/// typed expression, with the span of the source it comes from
/// (parentheses included); `e : ty` leaves `e` with the type `ty`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub ty: Type,
    pub span: Span,
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        self.span
    }
}

impl Pretty for Expression {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self.kind {
            ExpressionKind::Constant(c) => c.pretty(theme),
            ExpressionKind::Variable(var) => theme.expr_var(&var.value),
        }
    }
}
