use crate::lexer::TokenStream;
use cst::{Command, EndOfInput};
use errors::Errors;

pub mod parsing;

use parsing::parser;

pub enum CommandOrEnd {
    Command(Box<Command>),
    End(EndOfInput),
}

/// parse tokens
pub fn parse(tokens: TokenStream) -> Result<CommandOrEnd, Errors> {
    use chumsky::prelude::*;

    parser()
        .parse(tokens)
        .into_result()
        .map_err(|errs| errs.iter().map(crate::error::parsing).collect())
}
