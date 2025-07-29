use crate::utils::location::{Located, Location};
use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub enum Keyword {
    Definition,
    Eval,
    Type,
    TypeOf,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Colon,
    EqDef,
    LParen,
    RParen,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Comment(String),
    Identifier(String),
    Number(BigUint),
    Character(char),
    Keyword(Keyword),
    Operator(Operator),
    CommandEnd,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    loc: Location,
}

impl Token {
    pub fn new(kind: TokenKind, loc: Location) -> Self {
        Self { kind, loc }
    }
}

impl Located for Token {
    fn loc(&self) -> &Location {
        &self.loc
    }
}
