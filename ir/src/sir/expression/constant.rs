use crate::location::{Span, Spanned};
use crate::sir::Node;
use num_bigint::BigUint;
use pp::pretty::*;

// ============================================================================
// Builtin
// ============================================================================
pub use crate::common::BuiltinConstant;

// ============================================================================
// Constant
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    Builtin(Node<BuiltinConstant>),
    Number(Node<BigUint>),
    Character(Node<char>),
}

impl Pretty for Constant {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Constant::Builtin(b) => b.value.pretty(theme),
            Constant::Number(n) => theme.number(&n.value),
            Constant::Character(c) => theme.character(c.value),
        }
    }
}

impl Spanned for Constant {
    fn span(&self) -> Span {
        match self {
            Constant::Number(n) => n.span(),
            Constant::Character(c) => c.span(),
            Constant::Builtin(b) => b.span(),
        }
    }
}
