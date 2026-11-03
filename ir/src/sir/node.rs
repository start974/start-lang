use crate::location::{Span, Spanned};

// ============================================================================
// Node
// ============================================================================

/// a value with its span
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Spanned for Node<T> {
    fn span(&self) -> Span {
        self.span
    }
}
