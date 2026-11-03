use crate::location::{Span, Spanned};
use crate::sir::{Constant, DefId, Node, Type};

pub mod constant;
pub mod definition;
pub mod pattern;

// ============================================================================
// Expression
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    /// use of a definition, resolved by the lowering
    Variable(Node<DefId>),

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

impl Spanned for Expression {
    fn span(&self) -> Span {
        self.span
    }
}
