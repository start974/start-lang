use crate::pir::{Meta, Variable};
use location::{Span, Spanned};
use pp::pretty::*;

pub mod definition;

// ============================================================================
// Builtin
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
            Type::Variable(var) => var.pretty_with(theme, theme.ty_var(&var.value)),
            Type::Builtin(builtin) => builtin.pretty(theme),
        }
    }
}

impl Spanned for Type {
    fn span(&self) -> Span {
        match self {
            Type::Variable(var) => var.span(),
            Type::Builtin(builtin) => builtin.span(),
        }
    }
}
