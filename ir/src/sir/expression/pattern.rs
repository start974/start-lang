use crate::sir::Variable;
use location::{Span, Spanned};

// ============================================================================
// Pattern
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Variable(Variable),
}

impl Spanned for Pattern {
    fn span(&self) -> Span {
        match self {
            Pattern::Variable(var) => var.span(),
        }
    }
}
