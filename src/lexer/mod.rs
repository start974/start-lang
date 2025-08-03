use crate::utils::location::SourceId;
use chumsky::prelude::*;
use chumsky::text::inline_whitespace;
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

    let token_dot = just('.')
        .to(Token::Operator(token::Operator::Dot))
        .with_meta(source_id.clone(), offset)
        .lazy();
    let token_end = end()
        .to(Token::EndOfInput)
        .with_meta(source_id.clone(), offset);
    (choice((
        lexing::operator().map(Token::Operator),
        lexing::identifier().map(Token::Identifier),
        lexing::number().map(Token::Number),
        lexing::character().map(Token::Character),
    ))
    .padded()
    .with_meta(source_id.clone(), offset))
    .repeated()
    .collect::<Vec<_>>()
    .then(choice((token_dot, token_end)))
    .map(move |(mut tokens, end)| {
        tokens.push(end);
        tokens
    })
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
