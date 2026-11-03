//! Surface IR: untyped abstract syntax tree.
//!
//! Built from `pir` by the lowering: one node per construct, a span on every
//! node, no comments, no parentheses and no keyword variants. Every
//! definition gets a `DefId`, and every use is resolved to it (`Node<DefId>`).

pub mod command;
pub mod definition;
pub mod expression;
pub mod file;
pub mod node;
pub mod ty;

pub use crate::common::Documentation;
pub use command::Command;
pub use command::CommandKind;
pub use definition::{DefId, DefInfo, DefKind, FileId};
pub use expression::Expression;
pub use expression::ExpressionKind;
pub use expression::constant::Constant;
pub use expression::definition::ExpressionDefinition;
pub use expression::pattern::Pattern;
pub use file::File;
pub use node::Node;
pub use ty::Type;
pub use ty::definition::TypeDefinition;
