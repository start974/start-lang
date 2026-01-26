use cst::Meta;
use location::{GetSpan, Span};
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

pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn last_span(&self) -> Span {
        self.tokens.last().unwrap().span()
    }

    pub fn last_offset(&self) -> usize {
        self.last_span().end()
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl From<Vec<Token>> for TokenStream {
    fn from(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl Pretty for TokenStream {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(self.tokens.iter().map(|t| t.pretty(theme)), Doc::hardline()).group()
        //.append({
        //let span = self.span();
        //let start = span.start().to_string();
        //let end = span.end().to_string();
        //Doc::text("[")
        //.append(Doc::text(start))
        //.append(Doc::text(", "))
        //.append(Doc::text(end))
        //.append(Doc::text("]"))
        //})
    }
}

impl GetSpan for TokenStream {
    fn span(&self) -> Span {
        if let Some(first) = self.tokens.first()
            && let Some(last) = self.tokens.last()
        {
            first.span().union(last.span())
        } else {
            unreachable!("stram has no tokens")
        }
    }
}

