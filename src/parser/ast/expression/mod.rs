use super::identifier::Identifier;
use super::{Comment, WithComment};
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

mod constant;
mod definition;
pub mod kind;
mod type_restriction;

pub use constant::{Constant, ConstantKind};
pub use definition::Definition as ExpressionDefinition;
pub use kind::Kind as ExpressionKind;
pub use type_restriction::TypeRestriction;

#[derive(Debug)]
pub struct Expression {
    pub kind: kind::Kind,
    comments_before: Vec<Comment>,
    comments_after: Vec<Comment>,
}

impl From<ExpressionKind> for Expression {
    fn from(kind: ExpressionKind) -> Self {
        Self {
            kind,
            comments_before: Vec::new(),
            comments_after: Vec::new(),
        }
    }
}

impl From<Constant> for Expression {
    fn from(constant: Constant) -> Self {
        Self::from(ExpressionKind::Constant(constant))
    }
}

impl From<Identifier> for Expression {
    fn from(variable: Identifier) -> Self {
        Self::from(ExpressionKind::Variable(variable))
    }
}

impl From<TypeRestriction> for Expression {
    fn from(ty_restr: TypeRestriction) -> Self {
        Self::from(ExpressionKind::TypeRestriction(ty_restr))
    }
}

impl Located for Expression {
    fn loc(&self) -> &Location {
        self.kind.loc()
    }
}

impl WithComment for Expression {
    fn add_comment_before(&mut self, comment: Comment) {
        self.comments_before.push(comment);
    }

    fn add_comment_after(&mut self, comment: Comment) {
        self.comments_after.push(comment);
    }
}

impl Pretty for Expression {
    fn pretty(&self, theme: &Theme) -> Doc {
        self.kind.pretty(theme)
    }
}
