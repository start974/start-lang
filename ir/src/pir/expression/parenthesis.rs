use crate::location::{Span, Spanned};
use crate::pir::operator::{LParen, RParen};
use pp::pretty::*;

// ============================================================================
// Parenthesized
// ============================================================================
#[derive(Debug, Clone)]
pub struct Paren<T> {
    l_paren: LParen,
    inner: T,
    r_paren: RParen,
}

impl<T> Paren<T> {
    /// Create a new parenthesized value
    pub fn new(l_paren: LParen, inner: T, r_paren: RParen) -> Self {
        Self {
            l_paren,
            inner,
            r_paren,
        }
    }

    /// Get the value inside the parentheses
    pub fn inner(&self) -> &T {
        &self.inner
    }
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
                .append(self.l_paren.pretty_meta(theme))
                .append(doc_val)
                .append(self.r_paren.pretty_meta(theme))
        }
    }
}
