use crate::location::{Span, Spanned};
use crate::sir::{DefId, Node};

// ============================================================================
// Pattern
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    /// the name defined
    Variable(Node<DefId>),
}

impl Spanned for Pattern {
    fn span(&self) -> Span {
        match self {
            Pattern::Variable(var) => var.span(),
        }
    }
}
