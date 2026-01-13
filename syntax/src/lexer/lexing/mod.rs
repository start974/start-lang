use super::token;
use chumsky::extra::Err;
use chumsky::prelude::*;
use cst::Comment;
use location::Span;
use num_bigint::BigUint;
use std::rc::Rc;

mod comment;
mod identifier;
mod number;
mod with_meta;
mod character;
mod operator;

pub use chumsky::prelude::Parser;
pub use comment::comment;
pub use identifier::identifier;
pub use number::{
    digit, digit_bin, digit_hex, digit_oct, number, number_bin, number_dec, number_hex, number_oct,
};
use character::character;
pub use with_meta::WithMeta;
pub use operator::operator;

pub type ErrorChumsky<'src> = chumsky::error::Rich<'src, char>;
pub type ExtraChumsky<'src> = chumsky::extra::Err<ErrorChumsky<'src>>;

// ===========================================================================
// Lexer
// ===========================================================================
/// make a lexing with offset to token until "." (end of a command)
/// return offset rest to lexing
pub fn lexer<'src>(
    offset: usize,
) -> impl Parser<'src, &'src str, Vec<token::MetaToken>, Err<ErrorChumsky<'src>>> {
    use token::Token;

    let token = choice((
        operator().map(Token::Operator),
        identifier().map(Token::Identifier),
        number().map(Token::Number),
        character().map(Token::Character),
    ))
    .with_meta(offset);

    let token_dot = just('.')
        .to(Token::Operator(token::Operator::Dot))
        .with_meta(offset)
        .lazy();

    let token_end = end().to(Token::EndOfInput).with_meta(offset);

    token
        .repeated()
        .collect::<Vec<_>>()
        .then(choice((token_dot, token_end)))
        .map(move |(mut tokens, end)| {
            tokens.push(end);
            tokens
        })
}

