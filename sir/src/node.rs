use location::{Span, Spanned};

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

impl Node<String> {
    /// get name of an identifier
    pub fn name(&self) -> &str {
        &self.value
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
    fn node_span_and_name() {
        let node = Node::new("x".to_string(), Span::new(2, 3));
        assert_eq!(node.name(), "x");
        assert_eq!(node.span(), Span::new(2, 3));
    }
}
