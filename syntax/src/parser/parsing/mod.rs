use location::Span;

use super::CommandOrEnd;

mod command;
mod expression;
mod operator;
mod pattern;
mod ty;

pub use crate::lexer::token::{Operator, Token, TokenKind, TokenStream};
pub use chumsky::prelude::*;

pub use command::*;
pub use expression::*;
pub use operator::*;
pub use pattern::*;
pub use ty::*;

pub type ErrorChumsky = chumsky::error::Rich<'static, Token, Span>;
pub type ExtraChumsky = chumsky::extra::Err<ErrorChumsky>;

// ===========================================================================
// End of input
// ===========================================================================

/// parse end of input
pub fn end_of_input() -> impl Parser<'static, TokenStream, cst::EndOfFile, ExtraChumsky> {
    use cst::file::EndOfFileT;
    select! {meta @ Meta{ value: TokenKind::EndOfInput, ..} =>
        meta.map(|_| EndOfFileT())
    }
    .labelled("")
}

// ===========================================================================
// End of input
// ===========================================================================

/// parse with lexer tokens
pub fn parser() -> impl Parser<'static, TokenStream, CommandOrEnd, ExtraChumsky> {
    let command = command().map(Box::new).map(CommandOrEnd::Command);
    let eoi = end_of_input().map(CommandOrEnd::End);
    choice((command, eoi))
}
