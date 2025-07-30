use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

mod constant;
mod definition;
mod pattern;
mod type_restriction;
mod variable;

pub use constant::{Constant, Kind as ConstantKind};
pub use definition::Definition as ExpressionDefinition;
pub use pattern::{
    Pattern, /*Variable as PatternVariable, */ VariableName as PatternVariableName,
};
pub use type_restriction::TypeRestriction;
pub use variable::{Variable, VariableName};

#[derive(Debug)]
pub enum Expression {
    Constant(Constant),
    Variable(Variable),
    TypeRestriction(TypeRestriction),
}

impl From<Constant> for Expression {
    fn from(constant: Constant) -> Self {
        Expression::Constant(constant)
    }
}

impl From<Variable> for Expression {
    fn from(variable: Variable) -> Self {
        Expression::Variable(variable)
    }
}

impl From<TypeRestriction> for Expression {
    fn from(ty_restr: TypeRestriction) -> Self {
        Expression::TypeRestriction(ty_restr)
    }
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
    fn pretty(&self, theme: &Theme) -> Doc {
        match self {
            Expression::Constant(constant) => constant.pretty(theme),
            Expression::Variable(variable) => variable.pretty(theme),
            Expression::TypeRestriction(ty_restr) => ty_restr.pretty(theme),
        }
    }
}
