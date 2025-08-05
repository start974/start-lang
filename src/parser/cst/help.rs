use crate::lexer::Meta;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

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
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.def_var(&self.0)
    }
}


