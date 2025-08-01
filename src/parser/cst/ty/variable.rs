use super::super::Identifier;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug)]
pub struct VariableName(String);

impl std::fmt::Display for VariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for VariableName {
    fn from(name: String) -> Self {
        VariableName(name)
    }
}

impl Pretty for VariableName {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.ty_var(&self.0)
    }
}

pub type Variable = Identifier<VariableName>;
