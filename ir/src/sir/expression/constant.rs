use crate::location::{Span, Spanned};
use crate::sir::Node;
use num_bigint::BigUint;

// ============================================================================
// Number
// ============================================================================
pub type Number = Node<BigUint>;

// ============================================================================
// Character
// ============================================================================
pub type Character = Node<char>;

// ============================================================================
// Builtin
// ============================================================================
pub use crate::common::BuiltinConstant as BuiltinT;

pub type Builtin = Node<BuiltinT>;

// ============================================================================
// Constant
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    Builtin(Builtin),
    Number(Number),
    Character(Character),
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
