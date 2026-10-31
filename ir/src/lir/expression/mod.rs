use crate::lir::{Constant, GlobalId};
use pp::pretty::*;

pub mod constant;

// ============================================================================
// Expression
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    /// constant value
    Constant(Constant),
    /// value of a global, defined before
    Global(GlobalId),
}

impl Pretty for Expression {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Expression::Constant(c) => c.pretty(theme),
            Expression::Global(id) => theme.expr_var(id),
        }
    }
}
