use crate::lexer::meta::Meta;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::AsIdentifier;

// ============================================================================
// Variable
// ============================================================================
#[derive(Debug)]
pub struct VariableT(String);
pub type Variable = Meta<VariableT>;

impl Pretty for VariableT {
    fn pretty(&self, theme: &Theme) -> Doc {
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

impl Located for Pattern {
    fn loc(&self) -> Location {
        match self {
            Pattern::Variable(var) => var.loc(),
        }
    }
}

impl Pretty for Pattern {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Pattern::Variable(var) => var.pretty(theme),
        }
    }
}
