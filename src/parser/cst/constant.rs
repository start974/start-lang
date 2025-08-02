use super::{AsCharacter, AsNumber};
use crate::lexer::meta::Meta;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

// ============================================================================
// Number
// ============================================================================
#[derive(Debug, Clone)]
pub struct NumberT(BigUint);
pub type Number = Meta<NumberT>;

impl From<BigUint> for NumberT {
    fn from(value: BigUint) -> Self {
        Self(value)
    }
}

impl AsNumber for NumberT {
    fn as_number(&self) -> &BigUint {
        &self.0
    }
}

impl Pretty for NumberT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.number(&self.0)
    }
}

// ============================================================================
// Character
// ============================================================================
#[derive(Debug, Clone)]
pub struct CharacterT(char);
pub type Character = Meta<CharacterT>;

impl From<char> for CharacterT {
    fn from(value: char) -> Self {
        Self(value)
    }
}

impl AsCharacter for CharacterT {
    fn as_character(&self) -> char {
        self.0
    }
}

impl Pretty for CharacterT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.character(self.0)
    }
}

// ============================================================================
// Constant
// ============================================================================

#[derive(Debug, Clone)]
pub enum Constant {
    Number(Number),
    Character(Character),
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

impl Located for Constant {
    fn loc(&self) -> Location {
        match self {
            Constant::Number(n) => n.loc(),
            Constant::Character(c) => c.loc(),
        }
    }
}

impl Pretty for Constant {
    fn pretty(&self, theme: &Theme) -> Doc {
        match self {
            Constant::Number(n) => n.pretty(theme),
            Constant::Character(c) => c.pretty(theme),
        }
    }
}
