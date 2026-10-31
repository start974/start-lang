use crate::{Node, Variable};
use location::{Span, Spanned};

// ============================================================================
// Builtin
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinT {
    Nat,
    Bool,
    Char,
}
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
