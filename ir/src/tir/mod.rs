//! Typed IR: typed abstract syntax tree.
//!
//! Built by `typing`: every expression has its type, every variable points to
//! its unique definition (`a__0`).

pub mod command;
pub mod env;
pub mod error;
pub mod expression;
pub mod file;
pub mod identifier;
pub mod ty;

pub use crate::common::Documentation;
pub use command::{Command, CommandKind};
pub use env::Env;
pub use env::Help;
pub use expression::Expression;
pub use expression::constant::{Constant, ConstantKind, NConst};
pub use expression::definition::ExpressionDefinition;
pub use expression::pattern::{Pattern, PatternVar};
pub use file::File;
pub use identifier::Identifier;
pub use ty::{Type, Typed, TypedMut};
