use crate::lexer::MetaToken;
use crate::utils::location::{Located as _, SourceId};
use chumsky::input::{Input as _, ValueInput};
use chumsky::span::SimpleSpan;
use chumsky::Parser;
use cst::Command;
pub use error::Error;

pub mod cst;
pub mod error;
pub mod parsing;

pub type ErrorChumsky<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, MetaToken>>;

pub enum CommandOrEnd {
    Command(Box<Command>),
    End(cst::file::EndOfFile),
}

/// parse with lexer tokens
pub fn parser<'tokens, I>() -> impl Parser<'tokens, I, CommandOrEnd, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use chumsky::prelude::*;
    use parsing::{command, end_of_input};

    let command = command().map(Box::new).map(CommandOrEnd::Command);
    let eoi = end_of_input().map(CommandOrEnd::End);
    choice((command, eoi))
}

/// parse tokens
pub fn parse(source_id: SourceId, tokens: &[MetaToken]) -> Result<CommandOrEnd, Vec<Error>> {
    let tokens_spanned = tokens
        .iter()
        .map(|token| (token.clone(), token.loc().to_simple_span()))
        .collect::<Vec<_>>();
    let span_end: SimpleSpan = tokens_spanned.last().unwrap().1;
    let input = tokens_spanned.map(span_end, |(t, s)| (t, s));

    let res = parser().parse(input).into_result().map_err(|errs| {
        errs.iter()
            .map(|e| Error::new(e.clone(), source_id.clone()))
            .collect()
    });
    res
}
