use super::ast;
use crate::utils::location::{Location, SourceId};
use chumsky::prelude::*;
use num_bigint::BigUint;
use std::rc::Rc;

// ===========================================================================
// Utils
// ===========================================================================
/// parse unicode alphabetic characters
fn letter<'src>() -> impl Parser<'src, &'src str, char> {
    any().filter(|c: &char| c.is_alphabetic())
}

/// parse ascii digits
fn digit<'src>() -> impl Parser<'src, &'src str, char> {
    any().filter(|c: &char| c.is_ascii_digit())
}

/// parse ascii hexadecimal digits
fn digit_hex<'src>() -> impl Parser<'src, &'src str, char> {
    any().filter(|c: &char| c.is_ascii_hexdigit())
}

/// parse ascii octal digits (0-7)
fn digit_oct<'src>() -> impl Parser<'src, &'src str, char> {
    digit().filter(|c: &char| *c != '8' && *c != '9')
}

/// parse ascii binary digits (0-1)
fn digit_bin<'src>() -> impl Parser<'src, &'src str, char> {
    any().filter(|c: &char| *c == '0' || *c == '1')
}

// ===========================================================================
// Identifier
// ===========================================================================

/// parse `"_"* letter  (letter | digit | _)* "'"*`
pub fn identifier<'src>(
    source_id: &SourceId,
) -> impl Parser<'src, &'src str, ast::Identifier> + use<'src, '_> {
    let letter = Rc::new(letter());
    let underscore = Rc::new(just('_'));
    let digit = digit();
    let quote = just('\'');

    (underscore.clone().repeated().collect::<String>())
        .then(letter.clone())
        .then(
            letter
                .or(digit)
                .or(underscore)
                .repeated()
                .collect::<String>(),
        )
        .then(quote.repeated().collect::<String>())
        .map_with(
            move |(((underscores, first_letter), mid), apostrophes), e| {
                let span = e.span();
                let name = format!("{}{}{}{}", underscores, first_letter, mid, apostrophes);
                let loc = Location::new(span.start, span.end, source_id.clone());
                ast::Identifier::new(&name, loc)
            },
        )
}

// ===========================================================================
// Number
// ===========================================================================
/// parse number `digit ( digit | _)* digit
fn number_dec<'src>() -> impl Parser<'src, &'src str, BigUint> {
    let digit = Rc::new(digit());
    let underscore = just('_');

    (digit.clone())
        .then(
            (digit.clone())
                .or(underscore)
                .repeated()
                .collect::<String>(),
        )
        .then(digit)
        .map(|((digit1, digits2), digit3)| {
            let digits2 = digits2.replace('_', "");
            let number_str = format!("{}{}{}", digit1, digits2, digit3);
            BigUint::parse_bytes(number_str.as_bytes(), 10).expect("Failed to parse number")
        })
}

/// parse number with a base
fn number_base<'src>(
    prefix_lower: char,
    prefix_upper: char,
    radix: u32,
    digit: impl Parser<'src, &'src str, char>,
) -> impl Parser<'src, &'src str, BigUint> {
    let prefix = just("0")
        .then(just(prefix_lower).or(just(prefix_upper)))
        .ignored();
    let digit = Rc::new(digit);
    let underscore = just('_');

    prefix
        .then(digit.clone())
        .then(
            (digit.clone())
                .or(underscore)
                .repeated()
                .collect::<String>(),
        )
        .then(digit)
        .map(move |((((), digit1), digits2), digit3)| {
            let digits2 = digits2.replace('_', "");
            let number_str = format!("{}{}{}", digit1, digits2, digit3);
            BigUint::parse_bytes(number_str.as_bytes(), radix).expect("Failed to parse number")
        })
}

/// parse hexadecimal number `"0" ("x" | "X") digit_hex ( digit_hex | _)* digit_hex
pub fn number_hex<'src>() -> impl Parser<'src, &'src str, BigUint> {
    number_base('x', 'X', 16, digit_hex())
}

/// parse octal number `"0" ("o" | "O") digit_oct ( digit_oct | _)* digit_oct`
pub fn number_oct<'src>() -> impl Parser<'src, &'src str, BigUint> {
    number_base('o', 'O', 8, digit_oct())
}

/// parse binary number `"0" ("b" | "B") digit_bin ( digit_bin | _)* digit_bin`
pub fn number_bin<'src>() -> impl Parser<'src, &'src str, BigUint> {
    number_base('b', 'B', 2, digit_bin())
}

// ===========================================================================
// Type
// ===========================================================================

/// parse type
/// ```
/// type :=
/// | identifier
/// ```
pub fn ty<'src>(source_id: &SourceId) -> impl Parser<'src, &'src str, ast::Type> + use<'src, '_> {
    let identifier = identifier(source_id);
    identifier.map(ast::Type::Var)
}

/// parse type definition
/// ```
/// type_definition := identifier ":=" type
/// ```
pub fn type_definition<'src>(
    source_id: &SourceId,
) -> impl Parser<'src, &'src str, ast::TypeDefinition> + use<'src, '_> {
    let name = identifier(source_id);
    let op_eq_def = just(":=").ignored();
    let ty = ty(source_id);
    name.then(op_eq_def)
        .then(ty)
        .map(move |((name, ()), ty)| ast::TypeDefinition::new(name, ty))
}
