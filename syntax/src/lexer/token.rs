pub use super::TokenStream;
use cst::Meta;
use num_bigint::BigUint;
use pp::pretty::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Colon,
    EqDef,
    LParen,
    RParen,
    Eval,
    TypeOf,
    Help,
    Dot,
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
            Operator::Help => write!(f, "?"),
            Operator::Dot => write!(f, "."),
        }
    }
}

impl Pretty for Operator {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier(String),
    Number(BigUint),
    Character(char),
    Operator(Operator),
    EndOfInput,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Identifier(s) => write!(f, "{s}"),
            TokenKind::Number(n) => write!(f, "{n}"),
            TokenKind::Character(c) => write!(f, "'{c}'"),
            TokenKind::Operator(op) => write!(f, "{op}"),
            TokenKind::EndOfInput => write!(f, "end of input"),
        }
    }
}
impl Pretty for TokenKind {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            TokenKind::Identifier(s) => Doc::nil()
                .append(Doc::text("IDENTIFIER("))
                .append(Doc::text(s))
                .append(Doc::text(")"))
                .group(),
            TokenKind::Number(n) => Doc::nil()
                .append(Doc::text("NUMBER("))
                .append(theme.number(n))
                .append(Doc::text(")"))
                .group(),
            TokenKind::Character(c) => Doc::nil()
                .append(Doc::text("CHARACTER('"))
                .append(theme.character(*c))
                .append(Doc::text("')"))
                .group(),
            TokenKind::Operator(op) => Doc::nil()
                .append(Doc::text("OPERATOR("))
                .append(op.pretty(theme))
                .append(Doc::text(")"))
                .group(),
            TokenKind::EndOfInput => Doc::nil().append(Doc::text("END_OF_INPUT")).group(),
        }
    }
}

pub type Token = Meta<TokenKind>;
