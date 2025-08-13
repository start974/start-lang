use crate::utils::location::{Location, WithLoc};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

mod constant;
pub use constant::Constant;

// =========================================================================
// Value
// =========================================================================

#[derive(Debug, Clone)]
pub enum Value {
    Constant(Constant),
}

impl Value {
    /// make a value with location
    pub fn with_loc(self, loc: Location) -> WithLoc<Self> {
        WithLoc::new(loc, self)
    }
}

impl Pretty for Value {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Self::Constant(c) => c.pretty(theme),
        }
    }
}

impl From<Constant> for Value {
    fn from(c: Constant) -> Self {
        Value::Constant(c)
    }
}
