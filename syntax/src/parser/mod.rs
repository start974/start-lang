use crate::lexer::TokenStream;
use cst::{Command, EndOfFile};
use errors::Errors;

pub mod parsing;

use location::Span;
use parsing::parser;

pub enum CommandOrEnd {
    Command(Box<Command>),
    End(EndOfFile),
}

/// parse tokens
pub fn parse(tokens: TokenStream) -> Result<CommandOrEnd, Errors> {
    use chumsky::prelude::*;
    use location::GetSpan;

    let eoi: Span = tokens.last_span();
    let tokens_spanned = tokens
        .into_iter()
        .map(|token| {
            let span = token.span();
            (token.clone(), span)
        })
        .collect::<Vec<_>>();
    let input = tokens_spanned.map(eoi, |(t, s)| (t, s));

    parser()
        .parse(input)
        .into_result()
        .map_err(|errs| errs.iter().map(crate::error::parsing).collect())
}
