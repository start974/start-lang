use super::{token, ErrorChumsky};
use chumsky::prelude::*;
use num_bigint::BigUint;
use std::rc::Rc;

// ===========================================================================
// Commment
// ===========================================================================
/// lex comment
/// ```ebnf
/// COMMENT := "(*" <ANY>* "*)"
/// ```
pub fn comment<'src>() -> impl Parser<'src, &'src str, String, ErrorChumsky<'src>> {
    just("(*")
        .ignore_then(
            any()
                .and_is(just("*)").not())
                .repeated()
                .collect::<String>(),
        )
        .then_ignore(just("*)"))
        .labelled("comment")
}

// ===========================================================================
// Identifier
// ===========================================================================

/// lex identifier defined in
/// [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/) named `<IDENT>` in ebnf
/// follwing by quotes
/// ```ebnf
/// INDENTIFIER := <IDENT> "'"*
/// ```
pub fn identifier<'src>() -> impl Parser<'src, &'src str, String, ErrorChumsky<'src>> {
    text::unicode::ident()
        .then(just('\'').repeated().collect::<String>())
        .map(|(ident, quotes)| format!("{ident}{quotes}"))
        .labelled("identifier")
}

// ===========================================================================
// Number
// ===========================================================================
/// lex ascii digits
fn digit<'src>() -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
    any()
        .filter(|c: &char| c.is_ascii_digit())
        .labelled("digit")
}

/// lex ascii hexadecimal digits
fn digit_hex<'src>() -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
    any()
        .filter(|c: &char| c.is_ascii_hexdigit())
        .labelled("digit_hex")
}

/// lex ascii octal digits (0-7)
fn digit_oct<'src>() -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
    digit()
        .filter(|c: &char| *c != '8' && *c != '9')
        .labelled("digit_oct")
}

/// lex ascii binary digits (0-1)
fn digit_bin<'src>() -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
    any()
        .filter(|c: &char| *c == '0' || *c == '1')
        .labelled("digit_bin")
}

/// lex number with a base
/// digit ("_"* digit)*
fn number_f<'src>(
    radix: u32,
    digit: impl Parser<'src, &'src str, char, ErrorChumsky<'src>>,
) -> impl Parser<'src, &'src str, BigUint, ErrorChumsky<'src>> {
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
    digit: impl Parser<'src, &'src str, char, ErrorChumsky<'src>>,
) -> impl Parser<'src, &'src str, BigUint, ErrorChumsky<'src>> {
    let prefix = just("0").then(just(prefix_lower).or(just(prefix_upper)));
    prefix.ignore_then(number_f(radix, digit))
}

/// lex number `digit ( digit | _)*
fn number_dec<'src>() -> impl Parser<'src, &'src str, BigUint, ErrorChumsky<'src>> {
    number_f(10, digit()).labelled("number_dec")
}
/// lex hexadecimal number `"0" ("x" | "X") digit_hex ( digit_hex | _)*`
pub fn number_hex<'src>() -> impl Parser<'src, &'src str, BigUint, ErrorChumsky<'src>> {
    number_base_prefixed('x', 'X', 16, digit_hex()).labelled("number_hex")
}

/// lex octal number `"0" ("o" | "O") digit_oct ( digit_oct | _)*`
pub fn number_oct<'src>() -> impl Parser<'src, &'src str, BigUint, ErrorChumsky<'src>> {
    number_base_prefixed('o', 'O', 8, digit_oct()).labelled("number_oct")
}

/// lex binary number `"0" ("b" | "B") digit_bin ( digit_bin | _)*`
pub fn number_bin<'src>() -> impl Parser<'src, &'src str, BigUint, ErrorChumsky<'src>> {
    number_base_prefixed('b', 'B', 2, digit_bin()).labelled("number_bin")
}

/// lex number
pub fn number<'src>() -> impl Parser<'src, &'src str, BigUint, ErrorChumsky<'src>> {
    // lex decimal number or hexadecimal or octal or binary
    choice((number_hex(), number_oct(), number_bin(), number_dec())).labelled("number")
}

// ===========================================================================
// Character
// ===========================================================================

/// char with number
fn escape_number_char<'src>(
    digit: impl Parser<'src, &'src str, char, ErrorChumsky<'src>>,
    number_digit: usize,
    radix: u32,
) -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
    let digits = digit.repeated().exactly(number_digit).collect::<String>();

    digits.try_map(move |digits, span| {
        u8::from_str_radix(&digits, radix)
            .map(|b| b as char)
            .map_err(|_| Rich::custom(span, "Invalid escape character"))
    })
}

fn escape_number_char_prefixed<'src>(
    prefix: char,
    digit: impl Parser<'src, &'src str, char, ErrorChumsky<'src>>,
    number_digit: usize,
    radix: u32,
) -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
    just(prefix).ignore_then(escape_number_char(digit, number_digit, radix))
}

fn escape_unicode_char<'src>() -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
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
fn escape_char<'src>() -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
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
fn character_lit<'src>() -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
    choice((
        escape_char(),
        any().filter(|c: &char| {
            let cp = *c as u32;
            (cp <= 0xD7FF) || (0xE000..=0x10FFFF).contains(&cp)
        }),
    ))
    .labelled("character literal")
}

/// lex character
/// ```ebnf
/// CHARACTER := "'" CHARACTER_LIT "'"
/// ```
pub fn character<'src>() -> impl Parser<'src, &'src str, char, ErrorChumsky<'src>> {
    let quote = just('\'').labelled("'");
    character_lit()
        .delimited_by(quote, quote)
        .labelled("character")
}

// ===========================================================================
// Keyword
// ===========================================================================

fn command_keyword<'src>() -> impl Parser<'src, &'src str, token::Keyword, ErrorChumsky<'src>> {
    choice((
        choice((just("Definition"), just("Def"))).to(token::Keyword::Definition),
        choice((just("Eval"), just("$"))).to(token::Keyword::Eval),
        choice((just("Type"), just("Ty"))).to(token::Keyword::Type),
        choice((just("TypeOf"), just("?:"))).to(token::Keyword::TypeOf),
        just("Set").to(token::Keyword::Set(true)),
        just("UnSet").to(token::Keyword::Set(false)),
    ))
    .labelled("command keyword")
}

pub fn keyword<'src>() -> impl Parser<'src, &'src str, token::Keyword, ErrorChumsky<'src>> {
    command_keyword()
}

// ===========================================================================
// Operator
// ===========================================================================

/// colon operator
/// ```ebnf
/// COLON := ":"
/// ```
pub fn operator_colon<'src>() -> impl Parser<'src, &'src str, token::Operator, ErrorChumsky<'src>> {
    just(':').to(token::Operator::Colon)
}

/// definition operator
/// ```ebnf
/// EQ_DEF := ":="
/// ```
pub fn operator_eq_def<'src>() -> impl Parser<'src, &'src str, token::Operator, ErrorChumsky<'src>>
{
    just(":=").to(token::Operator::EqDef)
}

/// lparent operator
/// ```ebnf
/// LParent := "("
/// ```
pub fn operator_lparent<'src>() -> impl Parser<'src, &'src str, token::Operator, ErrorChumsky<'src>>
{
    just('(').to(token::Operator::LParen)
}
/// lparent operator
/// ```ebnf
/// RParent := "("
/// ```
pub fn operator_rparent<'src>() -> impl Parser<'src, &'src str, token::Operator, ErrorChumsky<'src>>
{
    just(')').to(token::Operator::RParen)
}

/// lex operator
pub fn operator<'src>() -> impl Parser<'src, &'src str, token::Operator, ErrorChumsky<'src>> {
    choice((
        operator_colon(),
        operator_eq_def(),
        operator_lparent(),
        operator_rparent(),
    ))
}
