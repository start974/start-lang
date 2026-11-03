use crate::location::{Span, Spanned};
use crate::sir::{Identifier, Node};
use pp::pretty::*;

// ============================================================================
// Pattern
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    /// the name defined
    Variable(Node<Identifier>),
}

impl Pretty for Pattern {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Pattern::Variable(var) => theme.def_var(&var.value),
        }
    }
}

impl Spanned for Pattern {
    fn span(&self) -> Span {
        match self {
            Pattern::Variable(var) => var.span(),
        }
    }
}
