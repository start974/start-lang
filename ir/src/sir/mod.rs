//! Surface IR: untyped abstract syntax tree.
//!
//! Built from `pir` by `lowering`: one node per construct, a span on every
//! node, no comments, no parentheses and no keyword variants. Every
//! definition gets a `DefId`; the `DefTable` of the `File` maps it to its
//! name, kind, place (file, span of the name and of the definition) and
//! documentation. Uses keep their name until name
//! resolution.

pub mod command;
pub mod documentation;
pub mod expression;
pub mod file;
pub mod identifier;
pub mod node;
pub mod ty;
pub mod variable;

pub use command::Command;
pub use command::CommandKind;
pub use documentation::Documentation;
pub use expression::Expression;
pub use expression::ExpressionKind;
pub use expression::constant::Constant;
pub use expression::definition::ExpressionDefinition;
pub use expression::pattern::Pattern;
pub use file::File;
pub use identifier::DefId;
pub use identifier::DefInfo;
pub use identifier::DefKind;
pub use identifier::DefTable;
pub use node::Node;
pub use ty::Type;
pub use ty::definition::TypeDefinition;
pub use variable::Variable;
