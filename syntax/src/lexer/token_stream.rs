use crate::lexer::Token;
use chumsky::input::{Input, ValueInput};
use location::{GetSpan, Span};
use pp::pretty::*;

pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn last_offset(&self) -> usize {
        self.tokens.last().unwrap().span().end()
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl FromIterator<Token> for TokenStream {
    fn from_iter<I: IntoIterator<Item = Token>>(iter: I) -> Self {
        Self {
            tokens: iter.into_iter().collect(),
        }
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

impl Input<'_> for TokenStream {
    type Span = Span;

    type Token = Token;

    type MaybeToken = Token;

    type Cursor = usize;

    type Cache = Self;

    fn begin(self) -> (Self::Cursor, Self::Cache) {
        (0, self)
    }

    fn cursor_location(cursor: &Self::Cursor) -> usize {
        *cursor
    }

    unsafe fn next_maybe(
        cache: &mut Self::Cache,
        cursor: &mut Self::Cursor,
    ) -> Option<Self::MaybeToken> {
        let val = cache.tokens.get(*cursor).cloned();
        *cursor += 1;
        val
    }

    unsafe fn span(cache: &mut Self::Cache, range: std::ops::Range<&Self::Cursor>) -> Self::Span {
        let get_span = |i: usize| cache.tokens.get(i).map(|token| token.span()).unwrap();
        let start = get_span(*range.start).start();
        let end = get_span(range.end - 1).end();
        Span::new(start, end)
    }
}

impl ValueInput<'_> for TokenStream {
    unsafe fn next(cache: &mut Self::Cache, cursor: &mut Self::Cursor) -> Option<Self::Token> {
        unsafe { <Self as Input>::next_maybe(cache, cursor) }
    }
}
