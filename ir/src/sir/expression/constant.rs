use crate::location::{Span, Spanned};
use crate::sir::Node;
use num_bigint::BigUint;

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

impl Spanned for Constant {
    fn span(&self) -> Span {
        match self {
            Constant::Number(n) => n.span(),
            Constant::Character(c) => c.span(),
            Constant::Builtin(b) => b.span(),
        }
    }
}
