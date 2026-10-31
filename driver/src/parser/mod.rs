use errors::Errors;
use ir::pir::{Command, EndOfFile};
use ir::token::{MetaToken, MetaTokenStream};

mod error;
mod parsing;

pub use parsing::parser;

pub enum CommandOrEnd {
    Command(Box<Command>),
    End(EndOfFile),
}

pub type ErrorChumsky<'tokens> = chumsky::error::Rich<'tokens, MetaToken>;

/// parse tokens
pub fn parse(tokens: MetaTokenStream) -> Result<CommandOrEnd, Errors> {
    use chumsky::prelude::*;
    use ir::location::Spanned;

    let last = tokens.last_span();
    let eoi = SimpleSpan::from(last.start()..last.end());
    let tokens_spanned = tokens
        .into_iter()
        .map(|token| {
            let span = token.span();
            (token, SimpleSpan::from(span.start()..span.end()))
        })
        .collect::<Vec<_>>();
    let input = tokens_spanned.map(eoi, |(t, s)| (t, s));

    parser()
        .parse(input)
        .into_result()
        .map_err(|errs: Vec<ErrorChumsky>| errs.iter().map(error::error_parsing).collect())
}
