//! Tokens: a value of the source, its span and the trivia written before it
//! (comments, blank lines). The lexer makes `Token<TokenKind>`s, pir keeps a
//! `Token<…>` for every part of the source (keywords, operators, names).

use crate::location::{Span, Spanned};
use num_bigint::BigUint;
use pp::pretty::*;

mod trivia;

pub use trivia::{Comment, DocComment, Trivia};

// ============================================================================
// Token
// ============================================================================

/// a value of the source, its span, and the trivia written before it
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<T> {
    /// trivia written before, in order
    pub leading: Vec<Trivia>,
    pub value: T,
    pub span: Span,
}

impl<T> Token<T> {
    /// a `Token` of `value` at `span`, with nothing before yet
    pub fn builder(value: T, span: Span) -> TokenBuilder<T> {
        TokenBuilder {
            leading: Vec::new(),
            value,
            span,
        }
    }

    /// the same place and comments, with another value
    pub fn with_value<U>(&self, value: U) -> Token<U> {
        Token {
            leading: self.leading.clone(),
            value,
            span: self.span,
        }
    }

    /// change the value, keep the place and comments
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Token<U> {
        Token {
            leading: self.leading,
            value: f(self.value),
            span: self.span,
        }
    }

    /// a comment is written before
    pub(crate) fn has_comment(&self) -> bool {
        self.leading
            .iter()
            .any(|item| !matches!(item, Trivia::BlankLines))
    }

    /// the comments and blank lines before, alone
    pub(crate) fn pretty_leading(&self, theme: &Theme) -> Doc<'_> {
        let mut last_is_comment = false;
        let mut doc = Doc::nil();
        for val in &self.leading {
            if last_is_comment {
                doc = doc.append(Doc::hardline());
            }
            match val {
                Trivia::Comment(comment) => {
                    doc = doc.append(comment.pretty(theme));
                    last_is_comment = true;
                }
                Trivia::DocComment(documentation) => {
                    doc = doc.append(documentation.pretty(theme));
                    last_is_comment = true;
                }
                Trivia::BlankLines => {
                    doc = doc.append(Doc::hardline());
                    last_is_comment = false;
                }
            }
        }
        doc
    }

    /// the comments before, then the value; `end_line`: a break after a
    /// comment
    pub(crate) fn pretty_with_end_line(&self, theme: &Theme, end_line: bool) -> Doc<'_>
    where
        T: Pretty,
    {
        self.pretty_value_with_end_line(theme, end_line, self.value.pretty(theme))
    }

    /// the comments before, then `value` as the document of the value
    pub(crate) fn pretty_with<'a>(&'a self, theme: &Theme, value: Doc<'a>) -> Doc<'a> {
        self.pretty_value_with_end_line(theme, true, value)
    }

    fn pretty_value_with_end_line<'a>(
        &'a self,
        theme: &Theme,
        end_line: bool,
        value: Doc<'a>,
    ) -> Doc<'a> {
        self.pretty_leading(theme)
            .append(match self.leading.last() {
                Some(Trivia::DocComment(_)) => Doc::hardline(),
                Some(Trivia::Comment(_)) if end_line => Doc::line(),
                Some(Trivia::Comment(_)) => Doc::line_(),
                _ => Doc::nil(),
            })
            .append(value)
    }
}

// ============================================================================
// Builder
// ============================================================================

/// a `Token` being built: its trivia is added in order
#[derive(Debug)]
pub struct TokenBuilder<T> {
    leading: Vec<Trivia>,
    value: T,
    span: Span,
}

impl<T> TokenBuilder<T> {
    /// add an item written before: blank lines once in a row, and a comment
    /// drops the blank lines alone before it
    pub fn item(mut self, item: Trivia) -> Self {
        match item {
            Trivia::BlankLines => {
                if !matches!(self.leading.last(), Some(Trivia::BlankLines)) {
                    self.leading.push(Trivia::BlankLines);
                }
            }
            comment => {
                if matches!(self.leading.as_slice(), [Trivia::BlankLines]) {
                    self.leading.pop();
                }
                self.leading.push(comment);
            }
        }
        self
    }

    /// add the items written before, in order
    pub fn items(self, items: impl IntoIterator<Item = Trivia>) -> Self {
        items.into_iter().fold(self, Self::item)
    }

    pub fn build(self) -> Token<T> {
        Token {
            leading: self.leading,
            value: self.value,
            span: self.span,
        }
    }
}

/// used by the parser errors to print the expected tokens
impl<T> std::fmt::Display for Token<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Spanned for Token<T> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Pretty for Token<T>
where
    T: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.pretty_with_end_line(theme, true)
    }
}

// ============================================================================
// Token kind
// ============================================================================

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

#[derive(Default)]
pub struct TokenStream {
    tokens: Vec<Token<TokenKind>>,
}

impl TokenStream {
    /// span of the last token, empty at 0 if there is none
    pub fn last_span(&self) -> Span {
        self.tokens.last().map_or_else(Span::default, Spanned::span)
    }

    /// the tokens, in order
    pub fn iter(&self) -> std::slice::Iter<'_, Token<TokenKind>> {
        self.tokens.iter()
    }

    /// end of the last token
    pub fn last_offset(&self) -> usize {
        self.last_span().end()
    }
}

impl IntoIterator for TokenStream {
    type Item = Token<TokenKind>;
    type IntoIter = std::vec::IntoIter<Token<TokenKind>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl From<Vec<Token<TokenKind>>> for TokenStream {
    fn from(tokens: Vec<Token<TokenKind>>) -> Self {
        Self { tokens }
    }
}

impl Pretty for TokenStream {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(self.tokens.iter().map(|t| t.pretty(theme)), Doc::hardline()).group()
    }
}

/// span from the first to the last token, empty at 0 if there is none
impl Spanned for TokenStream {
    fn span(&self) -> Span {
        match (self.tokens.first(), self.tokens.last()) {
            (Some(first), Some(last)) => first.span().union(last.span()),
            _ => Span::default(),
        }
    }
}
