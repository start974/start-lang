use location::Span;

use super::CommandOrEnd;

mod command;
mod expression;
mod utils;
mod pattern;
mod ty;

pub use crate::lexer::token::{Operator, Token, TokenKind, TokenStream};
pub use chumsky::prelude::*;

pub use command::*;
pub use expression::*;
pub use utils::*;
pub use pattern::*;
pub use ty::*;

pub type ErrorChumsky = chumsky::error::Rich<'static, Token, Span>;
pub type ExtraChumsky = chumsky::extra::Err<ErrorChumsky>;

/// parse with lexer tokens
pub fn parser() -> impl Parser<'static, TokenStream, CommandOrEnd, ExtraChumsky> {
    let command = command().map(Box::new).map(CommandOrEnd::Command);
    let eoi = end_of_input().map(CommandOrEnd::End);
    choice((command, eoi))
}
