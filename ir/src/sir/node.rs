use crate::location::{Span, Spanned};

// ============================================================================
// Node
// ============================================================================

/// a value with its span
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    pub value: T,
    span: Span,
}

impl<T> Node<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

impl<T> Spanned for Node<T> {
    fn span(&self) -> Span {
        self.span
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_keeps_value_and_span() {
        let node = Node::new('x', Span::new(2, 3));
        assert_eq!(node.value, 'x');
        assert_eq!(node.span(), Span::new(2, 3));
    }
}
