mod constant;
mod definition;
mod type_restriction;

pub use constant::*;
pub use definition::Definition as ExpressionDefinition;
pub use type_restriction::*;

use super::identifier::Identifier;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub enum Expression {
    Constant(Constant),
    Variable(Identifier),
    TypeRestriction(TypeRestriction),
}

impl Located for Expression {
    fn loc(&self) -> &Location {
        match self {
            Expression::Constant(constant) => constant.loc(),
            Expression::Variable(variable) => variable.loc(),
            Expression::TypeRestriction(ty_restr) => ty_restr.loc(),
        }
    }
}

impl Pretty for Expression {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Expression::Constant(constant) => constant.pretty(theme),
            Expression::Variable(variable) => theme.expr_var(variable),
            Expression::TypeRestriction(ty_restr) => ty_restr.pretty(theme),
        }
    }
}
