use super::ast::{self};
use crate::utils::location::{Location, SourceId};
use chumsky::{prelude::*, text::whitespace};
use num_bigint::BigUint;
use std::rc::Rc;

pub type Error<'a> = extra::Err<Rich<'a, char>>;

// ===========================================================================
// Utils
// ===========================================================================
/// parse ascii digits
fn digit<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    any()
        .filter(|c: &char| c.is_ascii_digit())
        .labelled("digit")
}

/// parse ascii hexadecimal digits
fn digit_hex<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    any()
        .filter(|c: &char| c.is_ascii_hexdigit())
        .labelled("digit_hex")
}

/// parse ascii octal digits (0-7)
fn digit_oct<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    digit()
        .filter(|c: &char| *c != '8' && *c != '9')
        .labelled("digit_oct")
}

/// parse ascii binary digits (0-1)
fn digit_bin<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    any()
        .filter(|c: &char| *c == '0' || *c == '1')
        .labelled("digit_bin")
}

// ===========================================================================
// Identifier
// ===========================================================================

/// parse identifier defined in
/// [Unicode Standard Annex #31](https://www.unicode.org/reports/tr31/)
/// follwing by quotes
pub fn identifier<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, ast::Identifier, Error<'src>> {
    text::unicode::ident()
        .then(just('\'').repeated().collect::<String>())
        .map(|(ident, quotes)| format!("{ident}{quotes}"))
        .map_with(move |name, e| {
            let span: SimpleSpan = e.span();
            let loc = Location::new(source_id.clone(), span.start, span.end).with_offset(offset);
            ast::Identifier::new(&name, loc)
        })
        .labelled("identifier")
}

// ===========================================================================
// Number
// ===========================================================================
/// parse number with a base
/// digit (digit | "_")*
fn number_base<'src>(
    radix: u32,
    digit: impl Parser<'src, &'src str, char, Error<'src>>,
) -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    let digit = Rc::new(digit);
    let underscore = just('_');

    digit
        .clone()
        .then(
            (digit.clone())
                .or(underscore)
                .repeated()
                .collect::<String>(),
        )
        .map(move |(digit1, digits2)| {
            let digits2 = digits2.replace('_', "");
            let number_str = format!("{digit1}{digits2}");
            BigUint::parse_bytes(number_str.as_bytes(), radix).expect("Failed to parse number")
        })
}

/// parse number with a base
fn number_base_prefixed<'src>(
    prefix_lower: char,
    prefix_upper: char,
    radix: u32,
    digit: impl Parser<'src, &'src str, char, Error<'src>>,
) -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    let prefix = just("0").then(just(prefix_lower).or(just(prefix_upper)));
    prefix.ignore_then(number_base(radix, digit))
}

/// parse number `digit ( digit | _)*
fn number_dec<'src>() -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    number_base(10, digit()).labelled("number_dec")
}
/// parse hexadecimal number `"0" ("x" | "X") digit_hex ( digit_hex | _)*`
pub fn number_hex<'src>() -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    number_base_prefixed('x', 'X', 16, digit_hex()).labelled("number_hex")
}

/// parse octal number `"0" ("o" | "O") digit_oct ( digit_oct | _)*`
pub fn number_oct<'src>() -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    number_base_prefixed('o', 'O', 8, digit_oct()).labelled("number_oct")
}

/// parse binary number `"0" ("b" | "B") digit_bin ( digit_bin | _)*`
pub fn number_bin<'src>() -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    number_base_prefixed('b', 'B', 2, digit_bin()).labelled("number_bin")
}

/// parse number
pub fn number<'src>() -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    // parse decimal number or hexadecimal or octal or binary
    choice((number_hex(), number_oct(), number_bin(), number_dec())).labelled("number")
}

// ===========================================================================
// Character
// ===========================================================================

/// char with number
fn escape_number_char<'src>(
    digit: impl Parser<'src, &'src str, char, Error<'src>>,
    number_digit: usize,
    radix: u32,
) -> impl Parser<'src, &'src str, char, Error<'src>> {
    let digits = digit.repeated().exactly(number_digit).collect::<String>();

    digits.try_map(move |digits, span| {
        u8::from_str_radix(&digits, radix)
            .map(|b| b as char)
            .map_err(|_| Rich::custom(span, "Invalid escape character"))
    })
}

fn escape_number_char_prefixed<'src>(
    prefix: char,
    digit: impl Parser<'src, &'src str, char, Error<'src>>,
    number_digit: usize,
    radix: u32,
) -> impl Parser<'src, &'src str, char, Error<'src>> {
    just(prefix).ignore_then(escape_number_char(digit, number_digit, radix))
}

fn escape_unicode_char<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
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
fn escape_char<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
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

/// parse caracter
/// ```ebnf
/// character_literal :=
/// | escape_char
/// | [U+0000 .. U+D7FF]
/// | [U+E000 .. U+10FFFF]
/// ```
fn character_litteral<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    choice((
        escape_char(),
        any().filter(|c: &char| {
            let cp = *c as u32;
            (cp <= 0xD7FF) || (0xE000..=0x10FFFF).contains(&cp)
        }),
    ))
    .labelled("character literal")
}

pub fn character<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    let quote = just('\'').labelled("'");
    character_litteral()
        .delimited_by(quote, quote)
        .labelled("character")
}

// ===========================================================================
// Expression
// ===========================================================================

/// parse type restriction
/// ```ebfn
/// type_restr := ":" type
///```
pub fn type_restriction<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, ast::Type, Error<'src>> {
    let op_colon = just(':');
    let ty = ty(source_id.clone(), offset);
    op_colon
        .padded()
        .ignore_then(ty)
        .labelled("type_restriction")
}

/// parse constant
/// ```ebfn
/// constant :=
/// | number
/// | character
///```
pub fn constant<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, ast::Constant, Error<'src>> {
    let number = {
        let source_id = source_id.clone();
        number().map_with(move |number, e| {
            let span = e.span();
            let loc = Location::new(source_id.clone(), span.start, span.end).with_offset(offset);
            ast::Constant::nat(number, loc)
        })
    };

    let character = {
        let source_id = source_id.clone();
        character().map_with(move |c, e| {
            let span = e.span();
            let loc = Location::new(source_id.clone(), span.start, span.end).with_offset(offset);
            ast::Constant::char(c, loc)
        })
    };

    choice((number, character)).labelled("constant")
}

/// parse expression
/// ```ebfn
/// expr0 :=
/// | "(" expr@1 ")"
/// | identifier
/// | constant
///
/// expr@1 :=
/// | expr@0 type_restiction
/// | expr@0
///```
pub fn expression<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, ast::Expression, Error<'src>> {
    recursive(move |expr1| {
        let expr0 = {
            let identifier = identifier(source_id.clone(), offset).map(ast::Expression::from);
            let constant = constant(source_id.clone(), offset).map(ast::Expression::from);
            let parens = expr1.padded().delimited_by(just('('), just(')'));
            choice((identifier, constant, parens)).boxed()
        };

        let expr1 = {
            let type_restriction = (expr0.clone())
                .padded()
                .then(type_restriction(source_id.clone(), offset))
                .map(|(expr, ty)| ast::TypeRestriction::new(expr, ty))
                .map(ast::Expression::from);
            choice((type_restriction, expr0)).boxed()
        };

        expr1
    })
    .labelled("expression")
}

/// parse expression definition
/// ```ebfn
///expr_definition := identifier type_rest? ":=" expr
///```
pub fn expression_definition<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, ast::ExpressionDefinition, Error<'src>> {
    let identifier = identifier(source_id.clone(), offset);
    let type_restriction = type_restriction(source_id.clone(), offset).or_not();
    let op_eq_def = just(":=").labelled(":=");
    let expr = expression(source_id, offset);
    identifier
        .padded()
        .then(type_restriction)
        .padded()
        .then_ignore(op_eq_def)
        .padded()
        .then(expr)
        .map(|((name, opt_ty), expr)| {
            let def = ast::ExpressionDefinition::new(name, expr);
            match opt_ty {
                Some(ty) => def.with_ty(ty),
                None => def,
            }
        })
}

// ===========================================================================
// Type
// ===========================================================================

/// parse type
/// ```ebfn
/// type :=
/// | identifier
/// ```
pub fn ty<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, ast::Type, Error<'src>> {
    let identifier = identifier(source_id, offset);
    identifier.map(ast::Type::from).labelled("type")
}

/// parse type definition
/// ```ebfn
/// type_definition := identifier ":=" type
/// ```
pub fn type_definition<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, ast::TypeDefinition, Error<'src>> {
    let name = identifier(source_id.clone(), offset);
    let op_eq_def = just(":=").labelled(":=");
    let ty = ty(source_id, offset);
    name.padded()
        .then_ignore(op_eq_def)
        .padded()
        .then(ty)
        .map(move |(name, ty)| ast::TypeDefinition::new(name, ty))
        .padded()
}

// ===========================================================================
// Command
// ===========================================================================
/// parse command
/// ```ebfn
/// command :=
/// | ("Definition" | "Def") SPACE expr_definition
/// | ("Type" | "Ty") SPACE type_definition
/// | ("Eval" | "$") SPACE expr
/// | ("TypeOf" | "?:") SPACE expr
///```
pub fn command<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, ast::CommandKind, Error<'src>> {
    let def_expr = (choice((just("Definition"), just("Def")))
        .labelled("Definition")
        .then(whitespace().at_least(1)))
    .ignore_then(expression_definition(source_id.clone(), offset))
    .map(ast::CommandKind::ExpressionDefinition);

    let def_type = (choice((just("Type"), just("Ty")))
        .labelled("Type")
        .then(whitespace().at_least(1)))
    .ignore_then(type_definition(source_id.clone(), offset))
    .map(ast::CommandKind::TypeDefinition);

    let eval = (choice((just("Eval"), just("$")))
        .labelled("Eval")
        .then(whitespace().at_least(1)))
    .ignore_then(expression(source_id.clone(), offset))
    .map(ast::CommandKind::Eval);

    let type_of = (choice((just("TypeOf"), just("?:")))
        .labelled("TypeOf")
        .then(whitespace().at_least(1)))
    .ignore_then(expression(source_id.clone(), offset))
    .map(ast::CommandKind::TypeOf);

    let set = (just("Set").labelled("Set").then(whitespace().at_least(1)))
        .ignore_then(identifier(source_id.clone(), offset))
        .map(|id| ast::CommandKind::Set(true, id));

    let unset = (just("Unset")
        .labelled("Unset")
        .then(whitespace().at_least(1)))
    .ignore_then(identifier(source_id, offset))
    .map(|id| ast::CommandKind::Set(false, id));

    choice((def_expr, def_type, eval, type_of, set, unset)).labelled("command")
}

/// parse command with dot
/// ```ebfn
/// command_dot := command "."
///```
pub fn command_dot<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, ast::Command, Error<'src>> {
    let cmd_kind = command(source_id.clone(), offset);
    let dot = just('.').labelled(".");
    cmd_kind
        .padded()
        .then_ignore(dot)
        .map_with(move |cmd_kind, e| {
            let span: SimpleSpan = e.span();
            let loc = Location::new(source_id.clone(), span.start, span.end).with_offset(offset);
            ast::Command::new(cmd_kind, loc)
        })
}

/// parse command and return also offset of end
pub fn command_offset<'src>(
    source_id: SourceId,
    offset: usize,
) -> impl Parser<'src, &'src str, (ast::Command, usize), Error<'src>> {
    command_dot(source_id, offset)
        .padded()
        .map_with(|cmd, e| (cmd, e.span().end))
        .then_ignore(any().repeated())
        .then_ignore(end())
}
