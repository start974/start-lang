use chumsky::span::SimpleSpan;
use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Colon,
    EqDef,
    LParen,
    RParen,
    Eval,
    TypeOf,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Colon => write!(f, ":"),
            Operator::EqDef => write!(f, ":="),
            Operator::LParen => write!(f, "("),
            Operator::RParen => write!(f, ")"),
            Operator::Eval => write!(f, "$"),
            Operator::TypeOf => write!(f, "?:"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Comment(String),
    Identifier(String),
    Number(BigUint),
    Character(char),
    Operator(Operator),
    CommandEnd,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Comment(s) => write!(f, "(* {s} *)"),
            Token::Identifier(s) => write!(f, "{s}"),
            Token::Number(n) => write!(f, "{n}"),
            Token::Character(c) => write!(f, "'{c}'"),
            Token::Operator(op) => write!(f, "{op}"),
            Token::CommandEnd => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenSpanned {
    pub token: Token,
    pub span: SimpleSpan,
}

impl std::fmt::Display for TokenSpanned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}
