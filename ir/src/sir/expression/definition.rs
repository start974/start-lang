use crate::sir::{DefId, Expression, Pattern, Type};
use location::{Span, Spanned};

// ============================================================================
// Expression Definition
// ============================================================================

/// `pattern [: ty] := body`; its name, kind and documentation are in the
/// `DefTable` of the file, under `id`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionDefinition {
    pub id: DefId,
    pub pattern: Pattern,
    pub ty: Option<Type>,
    pub body: Expression,
}

impl Spanned for ExpressionDefinition {
    fn span(&self) -> Span {
        self.pattern.span().union(self.body.span())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sir::{Constant, DefKind, DefTable, ExpressionKind, Node};
    use num_bigint::BigUint;

    #[test]
    fn expression_definition_span() {
        // `a := 3`
        let a = Node::new("a".to_string(), Span::new(0, 1));
        let def = ExpressionDefinition {
            id: DefTable::default().define(a.clone(), DefKind::Expression, Span::new(0, 6), None),
            pattern: Pattern::Variable(a),
            ty: None,
            body: Expression::new(
                ExpressionKind::Constant(Constant::Number(Node::new(
                    BigUint::from(3u8),
                    Span::new(5, 6),
                ))),
                Span::new(5, 6),
            ),
        };
        assert_eq!(def.span(), Span::new(0, 6));
    }
}
