use super::super::identifier::Identifier;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use super::{Constant, TypeRestriction};

#[derive(Debug)]
pub enum Kind {
    Constant(Constant),
    Variable(Identifier),
    TypeRestriction(TypeRestriction),
}

impl From<Constant> for Kind {
    fn from(constant: Constant) -> Self {
        Kind::Constant(constant)
    }
}

impl From<Identifier> for Kind {
    fn from(variable: Identifier) -> Self {
        Kind::Variable(variable)
    }
}

impl From<TypeRestriction> for Kind {
    fn from(ty_restr: TypeRestriction) -> Self {
        Kind::TypeRestriction(ty_restr)
    }
}

impl Located for Kind {
    fn loc(&self) -> &Location {
        match self {
            Kind::Constant(constant) => constant.loc(),
            Kind::Variable(variable) => variable.loc(),
            Kind::TypeRestriction(ty_restr) => ty_restr.loc(),
        }
    }
}

impl Pretty for Kind {
    fn pretty(&self, theme: &Theme) -> Doc {
        match self {
            Kind::Constant(constant) => constant.pretty(theme),
            Kind::Variable(variable) => theme.expr_var(variable),
            Kind::TypeRestriction(ty_restr) => ty_restr.pretty(theme),
        }
    }
}
