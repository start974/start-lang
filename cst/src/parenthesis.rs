use crate::Meta;
use location::{GetSpan, Span};

// ============================================================================
// parenthesized
// ============================================================================
#[derive(Debug, Clone)]
pub struct Parenthesed<Left, Val, Right> {
    pub l_paren: Meta<Left>,
    pub inner: Val,
    pub r_paren: Meta<Right>,
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

impl<Left, Val, Right> GetSpan for Parenthesed<Left, Val, Right> {
    fn span(&self) -> Span {
        self.l_paren.span().union(self.r_paren.span())
    }
}
