use crate::lexer::meta::Meta;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ============================================================================
// Variable
// ============================================================================
#[derive(Debug)]
pub struct VariableT(String);
pub type Variable = Meta<VariableT>;

impl Pretty for VariableT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.ty_var(&self.0)
    }
}

// ============================================================================
// Type
// ============================================================================
#[derive(Debug)]
pub enum Type {
    Variable(Variable),
}

impl Pretty for Type {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Type::Variable(var) => var.pretty(theme),
        }
    }
}

impl Located for Type {
    fn loc(&self) -> Location {
        match self {
            Type::Variable(var) => var.loc(),
        }
    }
}
