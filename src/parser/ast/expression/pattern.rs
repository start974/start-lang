use super::super::Identifier;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug)]
pub struct VariableName(String);

impl From<String> for VariableName {
    fn from(name: String) -> Self {
        VariableName(name)
    }
}

impl Pretty for VariableName {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.def_var(&self.0)
    }
}

impl std::fmt::Display for VariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Variable = Identifier<VariableName>;

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
    fn loc(&self) -> &Location {
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
