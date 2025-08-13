use super::AsIdentifier;
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

impl AsIdentifier for VariableT {
    /// get name of variable
    fn name(&self) -> &str {
        &self.0
    }
}

impl Pretty for VariableT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.expr_var(&self.0)
    }
}
