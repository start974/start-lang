use super::CommandOrEnd;
use location::Span;

mod command;
mod expression;
mod pattern;
mod ty;
mod utils;

pub use crate::lexer::token::{Operator, Token, TokenKind, TokenStream};
pub use chumsky::prelude::*;

pub use command::*;
pub use expression::*;
pub use pattern::*;
pub use ty::*;
pub use utils::*;

pub type ErrorChumsky = chumsky::error::Rich<'static, Token, Span>;
pub type ExtraChumsky = chumsky::extra::Err<ErrorChumsky>;

/// parse program
pub fn parser() -> impl Parser<'static, TokenStream, CommandOrEnd, ExtraChumsky> {
    let command = command().map(Box::new).map(CommandOrEnd::Command);
    let eoi = end_of_input().map(CommandOrEnd::End);
    choice((command, eoi))
}
