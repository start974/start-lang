use crate::utils::location::SourceId;
use chumsky::{prelude::*, text::inline_whitespace};
use lexing::WithMeta as _;

pub mod comment;
pub mod error;
pub mod lexing;
pub mod meta;
pub mod token;

pub use error::Error;
pub use meta::Meta;

pub type ErrorChumsky<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, char>>;
pub use token::MetaToken;

/// make a lexing with offset to token until "." (end of a command)
/// return offset rest to lexing
pub fn lexer<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, Vec<MetaToken>, ErrorChumsky<'src>> {
    use token::Token;

    let token_dot = just('.').to(Token::Operator(token::Operator::Dot)).lazy();
    let token_end = end().to(Token::EndOfInput);
    choice((
        lexing::operator().map(Token::Operator),
        lexing::identifier().map(Token::Identifier),
        lexing::number().map(Token::Number),
        lexing::character().map(Token::Character),
        token_dot,
        token_end,
    ))
    .with_meta(source_id.clone(), offset)
    .padded_by(inline_whitespace())
    .repeated()
    .collect::<Vec<_>>()
}

/// apply lexer on [source_id] with [offset] on [content]
pub fn lex<'src>(
    source_id: SourceId,
    offset: usize,
    content: &'src str,
) -> Result<Vec<MetaToken>, Vec<Error<'src>>> {
    lexer(source_id.clone(), offset)
        .parse(content)
        .into_result()
        .map_err(|errs| {
            errs.iter()
                .map(|e| Error::new(e.clone(), source_id.clone(), offset))
                .collect()
        })
}
