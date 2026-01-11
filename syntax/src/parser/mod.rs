use crate::lexer::{MetaToken, MetaTokenStream};
use cst::{Command, EndOfFile};
use errors::Errors;

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
    use location::Spanned;

    let eoi: SimpleSpan = tokens.last_simple_span();
    let tokens_spanned = tokens
        .into_iter()
        .map(|token| {
            let span = token.span();
            (
                token.clone(),
                SimpleSpan {
                    start: span.start(),
                    end: span.end(),
                    context: (),
                },
            )
        })
        .collect::<Vec<_>>();
    let input = tokens_spanned.map(eoi, |(t, s)| (t, s));

    parser()
        .parse(input)
        .into_result()
        .map_err(|errs: Vec<ErrorChumsky>| errs.iter().map(crate::error::parsing).collect())
}
