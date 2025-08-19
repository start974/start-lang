use crate::lexer::meta::Meta;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::AsIdentifier;

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
        theme.ty_var(&self.0)
    }
}

// ============================================================================
// Variable
// ============================================================================

#[derive(Debug, Clone)]
pub enum BuiltinT {
    Nat,
    Bool,
    Char,
}
pub type Builtin = Meta<BuiltinT>;

impl Pretty for BuiltinT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            BuiltinT::Nat => theme.ty_var(&"__Type_Nat__"),
            BuiltinT::Bool => theme.ty_var(&"__Type_Bool__"),
            BuiltinT::Char => theme.ty_var(&"__Type_Char__"),
        }
    }
}

// ============================================================================
// Type
// ============================================================================
#[derive(Debug, Clone)]
pub enum Type {
    Builtin(Builtin),
    Variable(Variable),
}

impl Pretty for Type {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Type::Variable(var) => var.pretty(theme),
            Type::Builtin(builtin) => builtin.pretty(theme),
        }
    }
}

impl Located for Type {
    fn loc(&self) -> Location {
        match self {
            Type::Variable(var) => var.loc(),
            Type::Builtin(builtin) => builtin.loc(),
        }
    }
}
