use crate::sir::{Node, Variable};
use location::{Span, Spanned};

pub mod definition;

// ============================================================================
// Builtin
// ============================================================================
pub use crate::common::BuiltinType as BuiltinT;
pub type Builtin = Node<BuiltinT>;

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
