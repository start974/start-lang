use crate::location::{Span, Spanned};
use crate::sir::{DefId, Node};

pub mod definition;

// ============================================================================
// Builtin
// ============================================================================

pub use crate::common::BuiltinType;

// ============================================================================
// Type
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Builtin(Node<BuiltinType>),
    /// use of a type definition, resolved by the lowering
    Variable(Node<DefId>),
}

impl Spanned for Type {
    fn span(&self) -> Span {
        match self {
            Type::Variable(var) => var.span(),
            Type::Builtin(builtin) => builtin.span(),
        }
    }
}
