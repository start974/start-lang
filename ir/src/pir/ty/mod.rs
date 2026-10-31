use crate::location::{Span, Spanned};
use crate::pir::{Meta, Variable};
use pp::pretty::*;

pub mod definition;

// ============================================================================
// Builtin
// ============================================================================

/// payload of a builtin type token, shared with the other IRs
pub use crate::common::BuiltinType as BuiltinT;
pub type Builtin = Meta<BuiltinT>;

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
