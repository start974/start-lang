use super::ty::{Type, Typed, TypedMut};
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

mod constant;
mod definition;
mod variable;

pub use constant::{Constant, ConstantKind};
pub use definition::Definition as ExpressionDefinition;
pub use variable::{Variable as ExpressionVariable, VariableEnv};

// ==========================================================================
// Expression
// ==========================================================================

pub enum Expression {
    Constant(Constant),
    Variable(ExpressionVariable),
}

impl From<Constant> for Expression {
    fn from(constant: Constant) -> Self {
        Expression::Constant(constant)
    }
}

impl From<ExpressionVariable> for Expression {
    fn from(variable: ExpressionVariable) -> Self {
        Expression::Variable(variable)
    }
}

mod sealed_mut_ty {
    use super::*;
    impl TypedMut for Expression {
        fn ty_mut(&mut self) -> &mut Type {
            match self {
                Expression::Constant(c) => c.ty_mut(),
                Expression::Variable(v) => v.ty_mut(),
            }
        }
    }
}

impl Typed for Expression {
    fn ty(&self) -> &Type {
        match self {
            Expression::Constant(c) => c.ty(),
            Expression::Variable(v) => v.ty(),
        }
    }
}

impl Located for Expression {
    fn loc(&self) -> Location {
        match self {
            Expression::Constant(c) => c.loc(),
            Expression::Variable(v) => v.loc(),
        }
    }
}

impl Pretty for Expression {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Expression::Constant(c) => c.pretty(theme),
            Expression::Variable(v) => v.pretty(theme),
        }
    }
}
