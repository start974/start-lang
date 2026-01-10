#![feature(trait_alias)]

use cst::{Command, EndOfFile};
use errors::Errors;
use lexer::MetaTokenStream;

mod error;
mod extra;
mod parsing;

pub use extra::ErrorChumsky;
pub use extra::Parser;
pub use parsing::parser;

pub enum CommandOrEnd {
    Command(Box<Command>),
    End(EndOfFile),
}

/// parse tokens
pub fn parse(tokens: MetaTokenStream) -> Result<CommandOrEnd, Errors> {
    use chumsky::input::Stream;
    let stream = Stream::from_iter(tokens);
    parser()
        .parse(stream)
        .into_result()
        .map_err(|errs| errs.iter().map(error::error_parsing).collect())
}
