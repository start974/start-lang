use crate::{Lexer, token};
use chumsky::prelude::*;
use chumsky::text::{newline, whitespace};
use cst::{Comment, Meta, meta::CommentOrLines};
use location::Span;
use num_bigint::BigUint;
use std::rc::Rc;

// ===========================================================================
// Commment
// ===========================================================================
/// lex comment
/// ```ebnf
/// COMMENT := "(*" <ANY>* "*)"
/// ```
pub fn comment<'src>() -> impl Lexer<'src, Comment> {
    let start = just("(*")
        .ignore_then(just("*").or_not())
        .map(|opt| opt.is_some());

    start
        .then(
            any()
                .and_is(just("*)").not())
                .repeated()
                .collect::<String>(),
        )
        .then_ignore(just("*)"))
        .map(|(is_doc, str)| Comment::from(str).with_is_doc(is_doc))
        .labelled("comment")
}

// ===========================================================================
// Meta
// ===========================================================================
pub trait WithMeta<'src, T>: Lexer<'src, T> + Sized {
    /// meta(rule) = (LINE{2,} | WS* COMMENT)* WS* rule
    fn with_meta(self, offset: usize) -> impl Lexer<'src, Meta<T>> {
        let lines = newline().repeated().at_least(2).to(CommentOrLines::Lines);
        let comment = whitespace()
            .ignore_then(comment())
            .map(CommentOrLines::Comment);

        let meta_items = (lines.or(comment))
            .repeated()
            .collect::<Vec<CommentOrLines>>();

        // Consomme les derniers espaces/lignes avant le rule
        meta_items
            .then_ignore(whitespace())
            .then(self.map_with(move |value, e| {
                let span: chumsky::span::SimpleSpan = e.span();
                (value, Span::new(span.start, span.end).with_offset(offset))
            }))
            .map(move |(comments, (value, span))| Meta::new(value, span).with_items(&comments))
    }
}

impl<'src, T, L> WithMeta<'src, T> for L where L: Lexer<'src, T> + Sized {}

// ===========================================================================
// Identifier
// ===========================================================================

/// lex identifier defined in
/// [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/) named `<IDENT>` in ebnf
/// follwing by quotes
/// ```ebnf
/// INDENTIFIER := <IDENT> "'"*
/// ```
pub fn identifier<'src>() -> impl Lexer<'src, String> {
    text::unicode::ident()
        .then(just('\'').repeated().collect::<String>())
        .map(|(ident, quotes)| format!("{ident}{quotes}"))
        .labelled("identifier")
}

// ===========================================================================
// Number
// ===========================================================================
/// lex ascii digits
fn digit<'src>() -> impl Lexer<'src, char> {
    any()
        .filter(|c: &char| c.is_ascii_digit())
        .labelled("digit")
}

/// lex ascii hexadecimal digits
fn digit_hex<'src>() -> impl Lexer<'src, char> {
    any()
        .filter(|c: &char| c.is_ascii_hexdigit())
        .labelled("digit_hex")
}

/// lex ascii octal digits (0-7)
fn digit_oct<'src>() -> impl Lexer<'src, char> {
    digit()
        .filter(|c: &char| *c != '8' && *c != '9')
        .labelled("digit_oct")
}

/// lex ascii binary digits (0-1)
fn digit_bin<'src>() -> impl Lexer<'src, char> {
    any()
        .filter(|c: &char| *c == '0' || *c == '1')
        .labelled("digit_bin")
}

/// lex number with a base
/// digit ("_"* digit)*
fn number_f<'src>(radix: u32, digit: impl Lexer<'src, char>) -> impl Lexer<'src, BigUint> {
    let digit = Rc::new(digit);
    let underscores = just('_').repeated();

    digit
        .clone()
        .then(
            underscores
                .ignore_then(digit.clone())
                .repeated()
                .collect::<String>(),
        )
        .map(move |(digit1, digits2)| {
            let number_str = format!("{digit1}{digits2}");
            BigUint::parse_bytes(number_str.as_bytes(), radix).expect("Failed to parse number")
        })
}

/// lex number with a base
fn number_base_prefixed<'src>(
    prefix_lower: char,
    prefix_upper: char,
    radix: u32,
    digit: impl Lexer<'src, char>,
) -> impl Lexer<'src, BigUint> {
    let prefix = just("0").then(just(prefix_lower).or(just(prefix_upper)));
    prefix.ignore_then(number_f(radix, digit))
}

/// lex number `digit ( digit | _)*
fn number_dec<'src>() -> impl Lexer<'src, BigUint> {
    number_f(10, digit()).labelled("number_dec")
}
/// lex hexadecimal number `"0" ("x" | "X") digit_hex ( digit_hex | _)*`
pub fn number_hex<'src>() -> impl Lexer<'src, BigUint> {
    number_base_prefixed('x', 'X', 16, digit_hex()).labelled("number_hex")
}

/// lex octal number `"0" ("o" | "O") digit_oct ( digit_oct | _)*`
pub fn number_oct<'src>() -> impl Lexer<'src, BigUint> {
    number_base_prefixed('o', 'O', 8, digit_oct()).labelled("number_oct")
}

/// lex binary number `"0" ("b" | "B") digit_bin ( digit_bin | _)*`
pub fn number_bin<'src>() -> impl Lexer<'src, BigUint> {
    number_base_prefixed('b', 'B', 2, digit_bin()).labelled("number_bin")
}

/// lex number
pub fn number<'src>() -> impl Lexer<'src, BigUint> {
    // lex decimal number or hexadecimal or octal or binary
    choice((number_hex(), number_oct(), number_bin(), number_dec())).labelled("number")
}

// ===========================================================================
// Character
// ===========================================================================

/// char with number
fn escape_number_char<'src>(
    digit: impl Lexer<'src, char>,
    number_digit: usize,
    radix: u32,
) -> impl Lexer<'src, char> {
    let digits = digit.repeated().exactly(number_digit).collect::<String>();

    digits.try_map(move |digits, span| {
        u8::from_str_radix(&digits, radix)
            .map(|b| b as char)
            .map_err(|_| Rich::custom(span, "Invalid escape character"))
    })
}

fn escape_number_char_prefixed<'src>(
    prefix: char,
    digit: impl Lexer<'src, char>,
    number_digit: usize,
    radix: u32,
) -> impl Lexer<'src, char> {
    just(prefix).ignore_then(escape_number_char(digit, number_digit, radix))
}

fn escape_unicode_char<'src>() -> impl Lexer<'src, char> {
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
fn escape_char<'src>() -> impl Lexer<'src, char> {
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
fn character_lit<'src>() -> impl Lexer<'src, char> {
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
fn character<'src>() -> impl Lexer<'src, char> {
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
fn operator<'src>() -> impl Lexer<'src, token::Operator> {
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
pub fn lexer<'src>(offset: usize) -> impl Lexer<'src, Vec<token::MetaToken>> {
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
