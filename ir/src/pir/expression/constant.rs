use crate::location::{Span, Spanned};
use crate::pir::Meta;
use num_bigint::BigUint;
use pp::pretty::*;

// ============================================================================
// Number
// ============================================================================
pub type Number = Meta<BigUint>;

// ============================================================================
// Character
// ============================================================================
pub type Character = Meta<char>;

// ============================================================================
// Builtin
// ============================================================================
/// payload of a builtin constant token, shared with the other IRs
pub use crate::common::BuiltinConstant as BuiltinT;
pub type Builtin = Meta<BuiltinT>;

// ============================================================================
// Constant
// ============================================================================

#[derive(Debug, Clone)]
pub enum Constant {
    Builtin(Builtin),
    Number(Number),
    Character(Character),
}

impl From<Builtin> for Constant {
    fn from(value: Builtin) -> Self {
        Constant::Builtin(value)
    }
}

impl From<Number> for Constant {
    fn from(value: Number) -> Self {
        Constant::Number(value)
    }
}

impl From<Character> for Constant {
    fn from(value: Character) -> Self {
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
