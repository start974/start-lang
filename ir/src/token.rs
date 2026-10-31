//! Tokens: output of the lexer, input of the parser; each token keeps the
//! comments written before it (`Meta`).

use crate::location::{Span, Spanned};
use crate::pir::Meta;
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

pub struct MetaTokenStream {
    tokens: Vec<MetaToken>,
}

impl MetaTokenStream {
    /// span of the last token, empty at 0 if there is none
    pub fn last_span(&self) -> Span {
        self.tokens.last().map_or_else(Span::default, Spanned::span)
    }

    /// end of the last token
    pub fn last_offset(&self) -> usize {
        self.last_span().end()
    }
}

impl IntoIterator for MetaTokenStream {
    type Item = MetaToken;
    type IntoIter = std::vec::IntoIter<MetaToken>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl From<Vec<MetaToken>> for MetaTokenStream {
    fn from(tokens: Vec<MetaToken>) -> Self {
        Self { tokens }
    }
}

impl Pretty for MetaTokenStream {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(self.tokens.iter().map(|t| t.pretty(theme)), Doc::hardline()).group()
    }
}

/// span from the first to the last token; a stream always ends with
/// `EndOfInput`, so it is never empty
impl Spanned for MetaTokenStream {
    fn span(&self) -> Span {
        match (self.tokens.first(), self.tokens.last()) {
            (Some(first), Some(last)) => first.span().union(last.span()),
            _ => unreachable!("a token stream is never empty"),
        }
    }
}
