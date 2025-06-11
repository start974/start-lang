use super::super::Variable;
use crate::typing::ast::{Constant, Ty, Typed};
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Expression
// ==========================================================================

pub enum Expression {
    Const(Constant),
    Var(Variable),
}

impl Typed for Expression {
    fn ty(&self) -> &Ty {
        match self {
            Expression::Const(c) => c.ty(),
            Expression::Var(v) => v.ty(),
        }
    }
    fn ty_loc_mut(&mut self) -> &mut Location {
        match self {
            Expression::Const(c) => c.ty_loc_mut(),
            Expression::Var(v) => v.ty_loc_mut(),
        }
    }
}

impl Located for Expression {
    fn loc(&self) -> &Location {
        match self {
            Expression::Const(c) => c.loc(),
            Expression::Var(v) => v.loc(),
        }
    }
}

impl LocatedSet for Expression {
    fn set_loc(&mut self, loc: &impl Located) {
        match self {
            Expression::Const(c) => c.set_loc(loc),
            Expression::Var(v) => v.set_loc(loc),
        }
    }
}

impl Pretty for Expression {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Expression::Const(c) => c.pretty(theme),
            Expression::Var(v) => v.pretty(theme),
        }
    }
}
