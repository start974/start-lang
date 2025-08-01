use crate::lexer::meta::Meta;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

// ============================================================================
// Number
// ============================================================================
#[derive(Debug)]
pub struct NumberT(BigUint);
pub type Number = Meta<NumberT>;

impl Pretty for NumberT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.number(&self.0)
    }
}

// ============================================================================
// Character
// ============================================================================
#[derive(Debug)]
pub struct CharacterT(char);
pub type Character = Meta<CharacterT>;

impl Pretty for CharacterT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.character(self.0)
    }
}

// ============================================================================
// Constant
// ============================================================================

#[derive(Debug)]
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
