use super::{Documentation, Expression, Pattern, Type};
use location::{Span, Spanned};

// ============================================================================
// Expression Definition
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionDefinition {
    pub pattern: Pattern,
    pub ty: Option<Type>,
    pub body: Expression,
    pub doc: Option<Documentation>,
}

impl Spanned for ExpressionDefinition {
    fn span(&self) -> Span {
        self.pattern.span().union(self.body.span())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sir::expression::ExpressionKind;
    use crate::sir::{Constant, Node};
    use num_bigint::BigUint;

    #[test]
    fn expression_definition_span() {
        // `a := 3`
        let def = ExpressionDefinition {
            pattern: Pattern::Variable(Node::new("a".to_string(), Span::new(0, 1))),
            ty: None,
            body: Expression::new(
                ExpressionKind::Constant(Constant::Number(Node::new(
                    BigUint::from(3u8),
                    Span::new(5, 6),
                ))),
                Span::new(5, 6),
            ),
            doc: Some(Documentation::from(vec!["the answer".to_string()])),
        };
        assert_eq!(def.span(), Span::new(0, 6));
    }
}
