use super::ty::{Type, Typed, TypedMut};
use super::variable::Variable;
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

mod constant;
mod definition;

pub use constant::Constant;
pub use constant::ConstantKind;
pub use definition::Definition as ExpressionDefinition;

// ==========================================================================
// Expression
// ==========================================================================

pub enum Expression {
    Constant(Constant),
    Variable(Variable),
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
    fn loc(&self) -> &Location {
        match self {
            Expression::Constant(c) => c.loc(),
            Expression::Variable(v) => v.loc(),
        }
    }
}

impl LocatedSet for Expression {
    fn set_loc(&mut self, loc: &impl Located) {
        match self {
            Expression::Constant(c) => c.set_loc(loc),
            Expression::Variable(v) => v.set_loc(loc),
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
