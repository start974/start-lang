use super::ast;
use crate::utils::location::{Location, SourceId};
use chumsky::prelude::*;
use num_bigint::BigUint;
use std::rc::Rc;

pub type Error<'a> = extra::Err<Rich<'a, char>>;

// ===========================================================================
// Utils
// ===========================================================================
/// parse unicode alphabetic characters
fn letter<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    any().filter(|c: &char| c.is_alphabetic())
}

/// parse ascii digits
fn digit<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    any().filter(|c: &char| c.is_ascii_digit())
}

/// parse ascii hexadecimal digits
fn digit_hex<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    any().filter(|c: &char| c.is_ascii_hexdigit())
}

/// parse ascii octal digits (0-7)
fn digit_oct<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    digit().filter(|c: &char| *c != '8' && *c != '9')
}

/// parse ascii binary digits (0-1)
fn digit_bin<'src>() -> impl Parser<'src, &'src str, char, Error<'src>> {
    any().filter(|c: &char| *c == '0' || *c == '1')
}

// ===========================================================================
// Identifier
// ===========================================================================

/// parse `"_"* letter (letter | digit | _)* "'"*`
pub fn identifier<'src>(
    source_id: SourceId,
) -> impl Parser<'src, &'src str, ast::Identifier, Error<'src>> {
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
            let number_str = format!("{}{}", digit1, digits2);
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
    number_base(10, digit())
}
/// parse hexadecimal number `"0" ("x" | "X") digit_hex ( digit_hex | _)*`
pub fn number_hex<'src>() -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    number_base_prefixed('x', 'X', 16, digit_hex())
}

/// parse octal number `"0" ("o" | "O") digit_oct ( digit_oct | _)*`
pub fn number_oct<'src>() -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    number_base_prefixed('o', 'O', 8, digit_oct())
}

/// parse binary number `"0" ("b" | "B") digit_bin ( digit_bin | _)*`
pub fn number_bin<'src>() -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    number_base_prefixed('b', 'B', 2, digit_bin())
}

/// parse number
pub fn number<'src>() -> impl Parser<'src, &'src str, BigUint, Error<'src>> {
    // parse decimal number or hexadecimal or octal or binary
    choice((number_dec(), number_hex(), number_oct(), number_bin()))
}

// ===========================================================================
// Type
// ===========================================================================

/// parse type
/// ```
/// type :=
/// | identifier
/// ```
pub fn ty<'src>(source_id: SourceId) -> impl Parser<'src, &'src str, ast::Type, Error<'src>> {
    let identifier = identifier(source_id);
    identifier.map(ast::Type::Var)
}

/// parse type definition
/// ```
/// type_definition := identifier ":=" type
/// ```
pub fn type_definition<'src>(
    source_id: SourceId,
) -> impl Parser<'src, &'src str, ast::TypeDefinition, Error<'src>> {
    let name = identifier(source_id.clone());
    let op_eq_def = just(":=");
    let ty = ty(source_id);
    name.then_ignore(op_eq_def)
        .then(ty)
        .map(move |(name, ty)| ast::TypeDefinition::new(name, ty))
        .padded()
}

// ===========================================================================
// Expression
// ===========================================================================

/// parse type restriction
///```
/// type_restr := ":" type
///```
pub fn type_restriction<'src>(
    source_id: SourceId,
) -> impl Parser<'src, &'src str, ast::Type, Error<'src>> {
    let op_colon = just(':');
    let ty = ty(source_id.clone());
    op_colon.padded().ignore_then(ty)
}

/// parse constant
///```
/// constant :=
/// | number
///```
pub fn constant<'src>(
    source_id: SourceId,
) -> impl Parser<'src, &'src str, ast::Constant, Error<'src>> {
    let number = number().map_with(move |number, e| {
        let span = e.span();
        let loc = Location::new(span.start, span.end, source_id.clone());
        ast::Constant::n(number, loc)
    });
    number
}

/// parse expression
///```
/// expr :=
/// | "(" expr ")"
/// | identifier
/// | constant
/// | expr type_restiction
///```
pub fn expression<'src>(
    source_id: SourceId,
) -> impl Parser<'src, &'src str, ast::Expression, Error<'src>> {
    recursive(move |expr| {
        let identifier = Rc::new(identifier(source_id.clone()).map(ast::Expression::from));
        let constant = Rc::new(constant(source_id.clone()).map(ast::Expression::from));
        //let type_restriction = (expr0.clone())
        //.then(type_restriction(source_id))
        //.map(|(expr, ty)| ast::TypeRestriction::new(expr, ty))
        //.map(ast::Expression::from)
        //.padded();
        let parens = (just('('))
            .padded()
            .ignore_then(expr)
            .padded()
            .then_ignore(just(')'))
            .padded();

        choice((identifier, constant, parens))
    })
}

/// parse expression definition
///```
///expr_definition := identifier type_rest? ":=" expr
///```
pub fn expression_definition<'src>(
    source_id: SourceId,
) -> impl Parser<'src, &'src str, ast::ExpressionDefinition, Error<'src>> {
    let identifier = identifier(source_id.clone());
    let type_restriction = type_restriction(source_id.clone()).or_not();
    let op_eq_def = just(":=");
    let expr = expression(source_id);
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
// Command
// ===========================================================================
/// parse command
///```
/// command :=
/// | ("Definition" | "Def") expr_definition
/// | ("Type" | "Ty") type_definition
/// | ("Eval" | "$") expr
/// | ("TypeOf" | "?:") expr
///```
pub fn command<'src>(
    source_id: SourceId,
) -> impl Parser<'src, &'src str, ast::Command, Error<'src>> {
    let def_expr = (just("Definition").or(just("Def")))
        .padded()
        .ignore_then(expression_definition(source_id.clone()))
        .map(ast::Command::ExpressionDefinition);

    let def_type = (just("Type").or(just("Ty")))
        .ignore_then(type_definition(source_id.clone()))
        .map(ast::Command::TypeDefinition);

    let eval = (just("Eval").or(just("$")))
        .ignore_then(expression(source_id.clone()))
        .map(ast::Command::Eval);

    let type_of = (just("TypeOf").or(just("?:")))
        .ignore_then(expression(source_id))
        .map(ast::Command::TypeOf);

    choice((def_expr, def_type, eval, type_of)).padded()
}

/// parse command with dot
///```
/// command_dot := command "."
///```
pub fn command_dot<'src>(
    source_id: SourceId,
) -> impl Parser<'src, &'src str, ast::Command, Error<'src>> {
    let cmd = command(source_id);
    let dot = just('.');
    cmd.padded().then_ignore(dot).padded()
}

/// parse command and return also offset of end
pub fn command_offset<'src>(
    source_id: SourceId,
) -> impl Parser<'src, &'src str, (ast::Command, usize), Error<'src>> {
    command_dot(source_id)
        .map_with(|cmd, e| (cmd, e.span().end))
        .then_ignore(any().repeated())
}
