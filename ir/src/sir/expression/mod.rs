use crate::sir::{Constant, Type, Variable};
use location::{Span, Spanned};

pub mod constant;
pub mod definition;
pub mod pattern;

// ============================================================================
// Expression
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    /// variable
    Variable(Variable),

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
    span: Span,
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sir::ty;

    #[test]
    fn expression_keeps_its_span() {
        // `(a : Nat)`: the span includes the parentheses,
        // the variable keeps its own span
        let var = Variable::new("a".to_string(), Span::new(1, 2));
        let inner = Expression::new(ExpressionKind::Variable(var.clone()), Span::new(1, 2));
        let ty = Type::Builtin(ty::Builtin::new(ty::BuiltinT::Nat, Span::new(5, 8)));
        let expr = Expression::new(
            ExpressionKind::Typed {
                expr: Box::new(inner),
                ty,
            },
            Span::new(0, 9),
        );
        assert_eq!(expr.span(), Span::new(0, 9));
        assert_eq!(var.span(), Span::new(1, 2));
    }
}
