use crate::location::{Span, Spanned};
use crate::pir::Variable;
use pp::pretty::*;

// ============================================================================
// Pattern
// ============================================================================

#[derive(Debug)]
pub enum Pattern {
    Variable(Variable),
}

impl From<Variable> for Pattern {
    fn from(variable: Variable) -> Self {
        Pattern::Variable(variable)
    }
}

impl Pretty for Pattern {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Pattern::Variable(var) => var.pretty_with(theme, theme.def_var(&var.value)),
        }
    }
}

impl Spanned for Pattern {
    fn span(&self) -> Span {
        match self {
            Pattern::Variable(var) => var.span(),
        }
    }
}
