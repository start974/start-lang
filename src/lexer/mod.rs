use crate::utils::location::{Located as _, Location, SourceId};
use ariadne::Span as _;
use chumsky::prelude::*;

pub mod error;
pub mod lexing;
pub mod token;
pub use token::Token;

pub use error::Error;
pub type ErrorChumsky<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, char>>;

/// make a lexing with offset to token until "." (end of a command)
/// return offset rest to lexing
pub fn lexer<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, (Vec<token::Token>, usize), ErrorChumsky<'src>> {
    use token::{Token, TokenKind};
    let tokens = choice((
        lexing::comment().map(TokenKind::Comment),
        lexing::identifier().map(TokenKind::Identifier),
        lexing::number().map(TokenKind::Number),
        lexing::character().map(TokenKind::Character),
        lexing::keyword().map(TokenKind::Keyword),
        lexing::operator().map(TokenKind::Operator),
    ))
    .map_with({
        let source_id = source_id.clone();
        move |kind, e| {
            let span: SimpleSpan = e.span();
            Token::new(
                kind,
                Location::new(source_id.clone(), span.start, span.end).with_offset(offset),
            )
        }
    })
    .padded()
    .repeated()
    .at_least(1)
    .collect::<Vec<_>>();

    let end_command = just('.')
        .map_with(move |_, e| {
            let span: SimpleSpan = e.span();
            Token::new(
                TokenKind::CommandEnd,
                Location::new(source_id.clone(), span.start, span.end).with_offset(offset),
            )
        })
        .then_ignore(any().then(end()));

    let token_command = tokens.then(end_command).map(|(mut tokens, end)| {
        tokens.push(end.clone());
        (tokens, end.loc().end())
    });

    let empty_input = end().padded().map_with(|_, e| {
        let span: SimpleSpan = e.span();
        (Vec::new(), span.end)
    });

    choice((token_command, empty_input))
}

/// apply lexer on [source_id] with [offset] on [content]
pub fn lex<'src>(
    source_id: SourceId,
    offset: usize,
    content: &'src str,
) -> Result<(Vec<token::Token>, usize), Vec<Error<'src>>> {
    lexer(source_id.clone(), offset)
        .parse(content)
        .into_result()
        .map_err(|errs| {
            errs.iter()
                .map(|e| Error::new(e.clone(), source_id.clone(), offset))
                .collect()
        })
}
