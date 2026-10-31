use crate::Meta;
use location::{Span, Spanned};
use pp::pretty::*;

// ============================================================================
// parenthesized
// ============================================================================
#[derive(Debug, Clone)]
pub struct Parenthesed<Left, Val, Right> {
    l_paren: Meta<Left>,
    inner: Val,
    r_paren: Meta<Right>,
}

impl<Left, Val, Right> Parenthesed<Left, Val, Right> {
    /// Create a new parenthesed value
    pub fn new(l_paren: Meta<Left>, val: Val, r_paren: Meta<Right>) -> Self {
        Self {
            l_paren,
            inner: val,
            r_paren,
        }
    }

    /// Get the value inside the parentheses
    pub fn inner(&self) -> &Val {
        &self.inner
    }
}

impl<Left, Val, Right> Spanned for Parenthesed<Left, Val, Right> {
    fn span(&self) -> Span {
        self.l_paren.span().union(self.r_paren.span())
    }
}

impl<Left, Val, Right> PrettyPrecedence for Parenthesed<Left, Val, Right>
where
    Left: Pretty,
    Val: PrettyPrecedence,
    Right: Pretty,
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
