use chumsky::prelude::*;

pub mod error;
pub mod lexing;
pub mod token;

pub use error::Error;

use crate::utils::location::SourceId;
pub type ErrorChumsky<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, char>>;

/// make a lexing with offset to token until "." (end of a command)
/// return offset rest to lexing
pub fn lexer<'src>(
    offset: usize,
) -> impl Parser<'src, &'src str, Vec<token::TokenSpanned>, ErrorChumsky<'src>> {
    use token::{Token, TokenSpanned};
    let tokens = choice((
        lexing::comment().map(Token::Comment),
        lexing::identifier().map(Token::Identifier),
        lexing::number().map(Token::Number),
        lexing::character().map(Token::Character),
        lexing::keyword().map(Token::Keyword),
        lexing::operator().map(Token::Operator),
    ))
    .map_with(move |kind, e| {
        let span_e: SimpleSpan = e.span();
        let span = SimpleSpan {
            start: span_e.start + offset,
            end: span_e.end + offset,
            context: (),
        };
        TokenSpanned { token: kind, span }
    })
    .padded()
    .repeated()
    .at_least(1)
    .collect::<Vec<_>>();

    let end_command = just('.')
        .to(Token::CommandEnd)
        .map_with({
            move |kind, e| {
                let span_e: SimpleSpan = e.span();
                let span = SimpleSpan {
                    start: span_e.start + offset,
                    end: span_e.end + offset,
                    context: (),
                };
                TokenSpanned { token: kind, span }
            }
        })
        .padded()
        .then_ignore(any().then(end()));

    let token_command = tokens.then(end_command).map(|(mut tokens, end)| {
        tokens.push(end.clone());
        tokens
    });

    let empty_input = end().padded().map(|_| Vec::new());

    choice((token_command, empty_input))
}

/// apply lexer on [source_id] with [offset] on [content]
pub fn lex<'src>(
    source_id: SourceId,
    offset: usize,
    content: &'src str,
) -> Result<Vec<token::TokenSpanned>, Vec<Error<'src>>> {
    lexer(offset).parse(content).into_result().map_err(|errs| {
        errs.iter()
            .map(|e| Error::new(e.clone(), source_id.clone(), offset))
            .collect()
    })
}
