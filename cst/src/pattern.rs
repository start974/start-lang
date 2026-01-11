use crate::{AsIdentifier, Meta};
use location::{Span, Spanned};
use pp::pretty::*;

// ============================================================================
// Variable
// ============================================================================
#[derive(Debug, Clone)]
pub struct VariableT(String);
pub type Variable = Meta<VariableT>;

impl From<String> for VariableT {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl Pretty for VariableT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.def_var(&self.0)
    }
}

impl AsIdentifier for VariableT {
    /// get name of variable
    fn name(&self) -> &str {
        &self.0
    }
}

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
            Pattern::Variable(var) => var.pretty(theme),
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
