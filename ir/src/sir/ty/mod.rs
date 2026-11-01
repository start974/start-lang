use crate::location::{Span, Spanned};
use crate::sir::{Node, Variable};

pub mod definition;

// ============================================================================
// Builtin
// ============================================================================
pub use crate::common::BuiltinType;
pub type Builtin = Node<BuiltinType>;

// ============================================================================
// Type
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Builtin(Builtin),
    Variable(Variable),
}

impl Spanned for Type {
    fn span(&self) -> Span {
        match self {
            Type::Variable(var) => var.span(),
            Type::Builtin(builtin) => builtin.span(),
        }
    }
}
