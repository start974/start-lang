use crate::location::{Span, Spanned};
use crate::sir::{Identifier, Node};
use pp::pretty::*;

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
    Variable(Node<Identifier>),
}

impl Pretty for Type {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Type::Builtin(builtin) => builtin.value.pretty(theme),
            Type::Variable(var) => theme.ty_var(&var.value),
        }
    }
}

impl Spanned for Type {
    fn span(&self) -> Span {
        match self {
            Type::Variable(var) => var.span(),
            Type::Builtin(builtin) => builtin.span(),
        }
    }
}
