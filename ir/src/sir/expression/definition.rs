use crate::location::{Span, Spanned};
use crate::sir::{Expression, Pattern, Type};

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

impl Spanned for ExpressionDefinition {
    fn span(&self) -> Span {
        self.pattern.span().union(self.body.span())
    }
}
