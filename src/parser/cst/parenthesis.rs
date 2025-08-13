use crate::lexer::meta::Meta;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

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

impl<Left, Val, Right> Located for Parenthesed<Left, Val, Right> {
    fn loc(&self) -> Location {
        self.l_paren.loc().union(self.r_paren.loc())
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

// ============================================================================
// Level of expression
// ============================================================================
pub trait PrettyPrecedence {
    /// get level of type
    fn precedence(&self) -> u8;

    /// pretty with precedence
    fn pretty_precedence(&self, min_prec: u8, theme: &Theme) -> Doc<'_>;
}

impl<T> Pretty for T
where
    T: PrettyPrecedence,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.pretty_precedence(self.precedence(), theme)
    }
}

impl<T> PrettyPrecedence for Box<T>
where
    T: PrettyPrecedence,
{
    fn precedence(&self) -> u8 {
        self.as_ref().precedence()
    }

    fn pretty_precedence(&self, min_prec: u8, theme: &Theme) -> Doc<'_> {
        self.as_ref().pretty_precedence(min_prec, theme)
    }
}
