use super::Constant;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// =========================================================================
// Value
// =========================================================================

#[derive(Debug, Clone)]
pub enum Value {
    Constant(Constant),
}

impl Pretty for Value {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Self::Constant(c) => c.pretty(theme)
        }
    }
}

impl From<Constant> for Value {
    fn from(c: Constant) -> Self {
        Value::Constant(c)
    }
}
