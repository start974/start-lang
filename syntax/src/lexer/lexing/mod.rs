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

pub use chumsky::prelude::Parser;
pub use comment::comment;
pub use identifier::identifier;
pub use number::{
    digit, digit_bin, digit_hex, digit_oct, number, number_bin, number_dec, number_hex, number_oct,
};
pub use with_meta::WithMeta;

pub type ErrorChumsky<'src> = chumsky::error::Rich<'src, char>;
pub type ExtraChumsky<'src> = chumsky::extra::Err<ErrorChumsky<'src>>;

// ===========================================================================
// Character
// ===========================================================================

/// char with number
fn escape_number_char<'src>(
    digit: impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>>,
    number_digit: usize,
    radix: u32,
) -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    let digits = digit.repeated().exactly(number_digit).collect::<String>();

    digits.try_map(move |digits, span| {
        u8::from_str_radix(&digits, radix)
            .map(|b| b as char)
            .map_err(|_| Rich::custom(span, "Invalid escape character"))
    })
}

fn escape_number_char_prefixed<'src>(
    prefix: char,
    digit: impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>>,
    number_digit: usize,
    radix: u32,
) -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    just(prefix).ignore_then(escape_number_char(digit, number_digit, radix))
}

fn escape_unicode_char<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    just('u')
        .ignore_then(
            digit_hex()
                .repeated()
                .at_least(1)
                .collect::<String>()
                .delimited_by(just('{'), just('}')),
        )
        .try_map(|digits, span| {
            u32::from_str_radix(&digits, 16)
                .ok()
                .and_then(std::char::from_u32)
                .ok_or_else(|| Rich::custom(span, "invalid unicode escape"))
        })
}

/// escape char
///```ebnf
/// escape_char := "\"
///    ("\\" | "\"" | "\'" | "n" | "r" | "t"
///    | digit{3} | "x" digit_hex{2} | "o" digit_oct{3}
///    | "u{" digit_hex+ "}")
///```
fn escape_char<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    just('\\').ignore_then(choice((
        just('\\').to('\\'),
        just('\"').to('\"'),
        just('\'').to('\''),
        just('n').to('\n'),
        just('r').to('\r'),
        just('t').to('\t'),
        escape_number_char(digit(), 3, 10),
        escape_number_char_prefixed('x', digit_hex(), 2, 16),
        escape_number_char_prefixed('o', digit_oct(), 3, 8),
        escape_unicode_char(),
    )))
}

/// lex caracter
/// ```ebnf
/// character_literal :=
/// | ESCAPE_CHAR
/// | [U+0000 .. U+D7FF]
/// | [U+E000 .. U+10FFFF]
/// ```
fn character_lit<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    choice((
        escape_char(),
        any().filter(|c: &char| {
            let cp = *c as u32;
            (cp <= 0xD7FF) || (0xE000..=0x10FFFF).contains(&cp)
        }),
    ))
}

/// lex character
/// ```ebnf
/// CHARACTER := "'" CHARACTER_LIT "'"
/// ```
pub fn character<'src>() -> impl Parser<'src, &'src str, char, Err<ErrorChumsky<'src>>> {
    let quote = just('\'').labelled("'");
    character_lit()
        .delimited_by(quote, quote)
        .labelled("character")
}

// ===========================================================================
// Keyword
// ===========================================================================

// ===========================================================================
// Operator
// ===========================================================================
/// lex operators
pub fn operator<'src>() -> impl Parser<'src, &'src str, token::Operator, Err<ErrorChumsky<'src>>> {
    choice((
        just("?:").to(token::Operator::TypeOf),
        just("?").to(token::Operator::Help),
        just(":=").to(token::Operator::EqDef),
        just(':').to(token::Operator::Colon),
        just('$').to(token::Operator::Eval),
        just('(').to(token::Operator::LParen),
        just(')').to(token::Operator::RParen),
    ))
}

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
