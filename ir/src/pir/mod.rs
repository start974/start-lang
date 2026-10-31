pub mod command;
pub mod comment;
pub mod constant;
pub mod documentation;
pub mod expression;
pub mod expression_definition;
pub mod file;
pub mod help;
pub mod meta;
pub mod operator;
pub mod parenthesis;
pub mod pattern;
pub mod ty;
pub mod ty_definition;

pub use command::Command;
pub use command::CommandKind;
pub use comment::Comment;
pub use constant::Constant;
pub use documentation::Documentation;
pub use expression::Expression;
pub use expression_definition::ExpressionDefinition;
pub use file::EndOfFile;
pub use file::File;
pub use meta::Meta;
pub use pattern::Pattern;
pub use ty::Type;
pub use ty_definition::TypeDefinition;

pub trait AsIdentifier {
    /// get name of Identifier
    fn name(&self) -> &str;
}

pub trait AsNumber {
    /// get the number as a reference to BigUint
    fn as_number(&self) -> &num_bigint::BigUint;
}

pub trait AsCharacter {
    /// get character as a char
    fn as_character(&self) -> char;
}
