use crate::lexer::token;
use crate::utils::location::SourceId;
use cst::Command;
use chumsky::input::{Input as _, ValueInput};
use chumsky::span::SimpleSpan;
use chumsky::Parser;
pub use error::Error;

pub mod cst;
pub mod error;
pub mod parsing;

pub type ErrorChumsky<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, token::Token>>;

/// parse with lexer tokens
pub fn parser<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::Command, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = token::Token, Span = SimpleSpan>,
{
    parsing::command(source_id)
}

/// parse tokens
pub fn parse<'tokens>(
    source_id: SourceId,
    tokens: &'tokens [token::TokenSpanned],
) -> Result<Command, Vec<Error<'tokens>>> {
    let input = {
        let span_end = tokens.last().unwrap().span;
        tokens.map(span_end, move |token_spanned| {
            (&token_spanned.token, &token_spanned.span)
        })
    };

    parser(source_id.clone())
        .parse(input)
        .into_result()
        .map_err(|errs| {
            errs.iter()
                .map(|e| Error::new(e.clone(), source_id.clone()))
                .collect()
        })
}
