use super::comment::WithComments;

mod constant;
mod definition;
mod kind;
mod type_restriction;

pub use constant::{Constant, ConstantKind};
pub use definition::Definition as ExpressionDefinition;
pub use kind::Kind as ExpressionKind;
pub use type_restriction::TypeRestriction;
pub type Expression = WithComments<ExpressionKind>;
