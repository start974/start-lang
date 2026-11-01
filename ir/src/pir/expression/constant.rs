use crate::location::{Span, Spanned};
use crate::token::Token;
use num_bigint::BigUint;
use pp::pretty::*;

// ============================================================================
// Number
// ============================================================================

// ============================================================================
// Character
// ============================================================================

// ============================================================================
// Builtin
// ============================================================================
/// payload of a builtin constant token, shared with the other IRs
pub use crate::common::BuiltinConstant;

// ============================================================================
// Constant
// ============================================================================

#[derive(Debug, Clone)]
pub enum Constant {
    Builtin(Token<BuiltinConstant>),
    Number(Token<BigUint>),
    Character(Token<char>),
}

impl From<Token<BuiltinConstant>> for Constant {
    fn from(value: Token<BuiltinConstant>) -> Self {
        Constant::Builtin(value)
    }
}

impl From<Token<BigUint>> for Constant {
    fn from(value: Token<BigUint>) -> Self {
        Constant::Number(value)
    }
}

impl From<Token<char>> for Constant {
    fn from(value: Token<char>) -> Self {
        Constant::Character(value)
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

impl Pretty for Constant {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Constant::Number(n) => n.pretty_with(theme, theme.number(&n.value)),
            Constant::Character(c) => c.pretty_with(theme, theme.character(c.value)),
            Constant::Builtin(b) => b.pretty(theme),
        }
    }
}
