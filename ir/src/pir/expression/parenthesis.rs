use crate::location::{Span, Spanned};
use crate::pir::operator::{LParen, RParen};
use crate::token::Token;
use pp::pretty::*;

// ============================================================================
// Parenthesized
// ============================================================================
#[derive(Debug, Clone)]
pub struct Paren<T> {
    pub l_paren: Token<LParen>,
    pub inner: T,
    pub r_paren: Token<RParen>,
}

impl<T> Spanned for Paren<T> {
    fn span(&self) -> Span {
        self.l_paren.span().union(self.r_paren.span())
    }
}

impl<T> PrettyPrecedence for Paren<T>
where
    T: PrettyPrecedence,
{
    fn precedence(&self) -> u8 {
        self.inner.precedence()
    }

    fn pretty_precedence(&self, prec: u8, theme: &Theme) -> Doc<'_> {
        let val_prec = self.inner.precedence();
        let doc_val = self.inner.pretty_precedence(self.precedence(), theme);
        if prec < val_prec {
            Doc::nil()
                .append(self.l_paren.pretty(theme))
                .append(doc_val)
                .append(self.r_paren.pretty(theme))
        } else {
            Doc::nil()
                .append(self.l_paren.pretty_leading(theme))
                .append(doc_val)
                .append(self.r_paren.pretty_leading(theme))
        }
    }
}
