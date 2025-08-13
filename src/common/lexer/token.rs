use num_bigint::BigUint;

use crate::utils::{
    location::{Located, Location},
    pretty::Pretty,
    theme::{Doc, Theme},
};

use super::Meta;

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
pub enum Token {
    Identifier(String),
    Number(BigUint),
    Character(char),
    Operator(Operator),
    EndOfInput,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "{s}"),
            Token::Number(n) => write!(f, "{n}"),
            Token::Character(c) => write!(f, "'{c}'"),
            Token::Operator(op) => write!(f, "{op}"),
            Token::EndOfInput => write!(f, "end of input"),
        }
    }
}
impl Pretty for Token {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Token::Identifier(s) => Doc::nil()
                .append(Doc::text("IDENTIFIER("))
                .append(Doc::text(s))
                .append(Doc::text(")"))
                .group(),
            Token::Number(n) => Doc::nil()
                .append(Doc::text("NUMBER("))
                .append(theme.number(n))
                .append(Doc::text(")"))
                .group(),
            Token::Character(c) => Doc::nil()
                .append(Doc::text("CHARACTER('"))
                .append(theme.character(*c))
                .append(Doc::text("')"))
                .group(),
            Token::Operator(op) => Doc::nil()
                .append(Doc::text("OPERATOR("))
                .append(op.pretty(theme))
                .append(Doc::text(")"))
                .group(),
            Token::EndOfInput => Doc::nil().append(Doc::text("END_OF_INPUT")).group(),
        }
    }
}

pub type MetaToken = Meta<Token>;

impl Pretty for Vec<MetaToken> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.iter().map(|token| {
                Doc::nil()
                    .append(Doc::text("["))
                    .append(token.pretty(theme))
                    .append(Doc::text("]"))
            }),
            Doc::line(),
        )
    }
}

impl Located for Vec<MetaToken> {
    fn loc(&self) -> Location {
        if self.is_empty() {
            Location::unknown()
        } else if self.len() == 1 {
            self.first().unwrap().loc()
        } else {
            let first = self.first().unwrap().loc();
            let last = self.last().unwrap().loc();
            first.union(last)
        }
    }
}
