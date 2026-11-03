//! Typed IR: typed abstract syntax tree.
//!
//! Built from `sir` by `typing`: same layout as `sir`, a span on every node,
//! every expression has its type, and every use names its definition
//! (`Identifier`, shown `a__0`). Type definitions leave no command: they only
//! give a type to their name.

pub mod command;
pub mod definition;
pub mod expression;
pub mod file;
pub mod ty;

pub use crate::common::Documentation;
pub use crate::sir::{DefId, DefKind, Node};
pub use command::{Command, CommandKind, Help};
pub use definition::{DefInfo, Identifier};
pub use expression::Expression;
pub use expression::ExpressionKind;
pub use expression::constant::Constant;
pub use expression::definition::ExpressionDefinition;
pub use expression::pattern::Pattern;
pub use file::File;
pub use ty::Type;
