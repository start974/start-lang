use chumsky::input::ValueInput;
use chumsky::prelude::*;
use ir::token::{Operator, Token, TokenKind};

use ir::pir;

/// parser state of chumsky: errors are `Rich` errors on tokens
pub type Extra<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, Token<TokenKind>>>;

// ===========================================================================
// Operator
// ===========================================================================
pub fn operator<'tokens, I, Op>(
    op: Operator,
    res: Op,
) -> impl Parser<'tokens, I, Token<Op>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
    Op: Clone,
{
    select! {ref token @ Token{ value: TokenKind::Operator(ref token_op), ..} if token_op == &op =>
            token.with_value(res.clone())
    }
}

// ===========================================================================
// Pattern
// ===========================================================================

/// parse pattern
/// ```ebfn
/// pattern :=
/// | IDENTIFIER
///```
pub fn pattern<'tokens, I>() -> impl Parser<'tokens, I, pir::Pattern, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    let variable = variable()
        .labelled("pattern variable")
        .map(pir::Pattern::from);

    variable.labelled("pattern")
}

// ===========================================================================
// Expression
// ===========================================================================

/// parse constant
/// ```ebfn
/// constant :=
/// | NUMBER
/// | CHARACTER
///```
pub fn constant<'tokens, I>() -> impl Parser<'tokens, I, pir::Constant, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::expression::constant::{BuiltinConstant, Constant};
    let number = select! {ref token @ Token{ value: TokenKind::Number(ref n), ..} =>
            token.with_value(n.clone())
    }
    .map(Constant::from);

    let builtin = select! {
        ref token @ Token{ value: TokenKind::Identifier(ref b), ..} if b == "__Constant_true__" =>
            token.with_value(BuiltinConstant::True),
        ref token @ Token{ value: TokenKind::Identifier(ref b), ..} if b == "__Constant_false__" =>
            token.with_value(BuiltinConstant::False)
    }
    .map(Constant::from);

    let character = select! {ref token @ Token{ value: TokenKind::Character(ref c), ..} =>
        token.with_value(*c)
    }
    .map(Constant::from);

    choice((builtin, number, character)).labelled("constant")
}

/// parse variable
/// ```ebfn
/// variable := IDENTIFIER
///```
pub fn variable<'tokens, I>() -> impl Parser<'tokens, I, pir::Variable, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    select! {ref token @ Token{ value: TokenKind::Identifier(ref s), ..} =>
            token.with_value(s.clone())
    }
}

/// parse expression0
/// ```ebfn
/// expression0 :=
/// | constant
/// | variable
/// | "(" expression ")"
///```
fn expression0<'tokens, I>(
    expr: impl Parser<'tokens, I, pir::Expression, Extra<'tokens>>,
) -> impl Parser<'tokens, I, pir::Expression, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::Expression;
    use pir::expression::parenthesis::Paren;

    let constant = constant().map(Expression::Constant);
    let variable = variable().map(Expression::Variable);
    let parens = {
        let l_paren = operator(Operator::LParen, pir::operator::LParen).labelled("(");
        let r_paren = operator(Operator::RParen, pir::operator::RParen).labelled(")");
        l_paren
            .then(expr.map(Box::new))
            .then(r_paren)
            .map(|((l_paren, inner), r_paren)| Paren {
                l_paren,
                inner,
                r_paren,
            })
            .map(Expression::Paren)
    };

    choice((constant, variable, parens))
}

/// parse expression1
/// ```ebfn
/// expr@1 :=
/// | expr@0 COLON ty
/// | expr@0
///```
fn expression1<'tokens, I>(
    expr0: impl Parser<'tokens, I, pir::Expression, Extra<'tokens>> + Clone,
) -> impl Parser<'tokens, I, pir::Expression, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    let colon = operator(Operator::Colon, pir::operator::Colon).labelled(":");
    let type_restriction = (expr0.clone())
        .then(colon)
        .then(ty())
        .map(|((expr, colon), ty)| pir::Expression::Typed {
            expr: Box::new(expr),
            colon,
            ty,
        });

    choice((type_restriction, expr0))
}
/// parse expression
/// ```ebfn
/// expr0 :=
/// | variable
/// | constant
/// | "(" expr@1 ")"
///
/// expr@1 :=
/// | expr@0 colon ty
/// | expr@0
///```
pub fn expression<'tokens, I>() -> impl Parser<'tokens, I, pir::Expression, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    recursive(move |expr| {
        let expr0 = expression0(expr.clone()).boxed();
        expression1(expr0).boxed()
    })
    .labelled("expression")
}

/// parse expression definition
/// ```ebfn
/// expr_definition := pattern (colon type)? EQ_DEF expression
///```
pub fn expression_definition<'tokens, I>()
-> impl Parser<'tokens, I, pir::ExpressionDefinition, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::expression::definition::{ExpressionDefinition, TypedBy};

    let typed_by = {
        let colon = operator(Operator::Colon, pir::operator::Colon).labelled(":");
        colon.then(ty()).map(|(colon, ty)| TypedBy { colon, ty })
    };
    let eq_def = operator(Operator::EqDef, pir::operator::EqDef).labelled(":=");
    pattern()
        .then(typed_by.or_not())
        .then(eq_def)
        .then(expression())
        .map(
            |(((pattern, typed_by), eq_def), body)| ExpressionDefinition {
                pattern,
                typed_by,
                eq_def,
                body,
            },
        )
}

// ===========================================================================
// Type
// ===========================================================================
/// parse type
/// ```ebfn
/// type_variable := IDENTIFIER
/// ```
pub fn ty_variable<'tokens, I>() -> impl Parser<'tokens, I, pir::Variable, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    variable().labelled("type variable")
}

/// parse type builtin
pub fn ty_builtin<'tokens, I>()
-> impl Parser<'tokens, I, Token<pir::ty::BuiltinType>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    select! {
        ref token @ Token{ value: TokenKind::Identifier(ref s), ..} if s == "__Type_Nat__" =>
            token.with_value(pir::ty::BuiltinType::Nat),

        ref token @ Token{ value: TokenKind::Identifier(ref s), ..} if s == "__Type_Bool__" =>
            token.with_value(pir::ty::BuiltinType::Bool),

        ref token @ Token{ value: TokenKind::Identifier(ref s), ..} if s == "__Type_Char__" =>
            token.with_value(pir::ty::BuiltinType::Char),
    }
    .labelled("builtin variable")
}

/// parse type
/// ```ebfn
/// type :=
/// | type_variable
/// ```
pub fn ty<'tokens, I>() -> impl Parser<'tokens, I, pir::Type, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    let builtin = ty_builtin().map(pir::Type::Builtin);
    let var = ty_variable().map(pir::Type::Variable);

    choice((builtin, var)).labelled("type")
}

/// parse type definition
/// ```ebfn
/// type_definition := type_variable EQ_DEF type
/// ```
pub fn type_definition<'tokens, I>() -> impl Parser<'tokens, I, pir::TypeDefinition, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    let eq_def = operator(Operator::EqDef, pir::operator::EqDef).labelled(":=");
    ty_variable()
        .then(eq_def)
        .then(ty())
        .map(|((name, eq_def), ty)| pir::TypeDefinition { name, eq_def, ty })
}

// ===========================================================================
// Type
// ===========================================================================
/// parse type
/// ```ebfn
/// help_variable := IDENTIFIER
/// ```
pub fn help_variable<'tokens, I>() -> impl Parser<'tokens, I, pir::Variable, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    variable().labelled("help variable")
}
// ===========================================================================
// Command
// ===========================================================================

fn keyword_definition<'tokens, I>()
-> impl Parser<'tokens, I, Token<pir::command::keyword::DefinitionKeyword>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::command::keyword::DefinitionKeyword;
    select! {
        ref token @ Token{value: TokenKind::Identifier(ref id), ..} if id == "Def" =>
            token.with_value(DefinitionKeyword::Def),
        ref token @ Token{value: TokenKind::Identifier(ref id), ..} if id == "Definition" =>
            token.with_value(DefinitionKeyword::Definition),
    }
}

fn keyword_eval<'tokens, I>()
-> impl Parser<'tokens, I, Token<pir::command::keyword::EvalKeyword>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::command::keyword::EvalKeyword;
    select! {
        ref token @ Token{value: TokenKind::Identifier(ref id), ..} if id == "Eval" =>
            token.with_value(EvalKeyword::Eval),
        ref token @ Token{value: TokenKind::Operator(Operator::Eval), ..} =>
            token.with_value(EvalKeyword::EvalOp)
    }
}

fn keyword_type_of<'tokens, I>()
-> impl Parser<'tokens, I, Token<pir::command::keyword::TypeOfKeyword>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::command::keyword::TypeOfKeyword;
    select! {
        ref token @ Token{value: TokenKind::Identifier(ref id), ..} if id == "TypeOf" =>
            token.with_value(TypeOfKeyword::TypeOf),
        ref token @ Token{value: TokenKind::Operator(Operator::TypeOf), ..} =>
            token.with_value(TypeOfKeyword::TypeOfOp),
    }
}

fn keyword_help<'tokens, I>()
-> impl Parser<'tokens, I, Token<pir::command::keyword::HelpKeyword>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::command::keyword::HelpKeyword;
    select! {
        ref token @ Token{value: TokenKind::Identifier(ref id), ..} if id == "Help" =>
            token.with_value(HelpKeyword::Help),
        ref token @ Token{value: TokenKind::Operator(Operator::Help), ..} =>
            token.with_value(HelpKeyword::HelpOp),
    }
}

fn keyword_type<'tokens, I>()
-> impl Parser<'tokens, I, Token<pir::command::keyword::TypeKeyword>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::command::keyword::TypeKeyword;
    select! {
        ref token @ Token{value: TokenKind::Identifier(ref id), ..} if id == "Type" =>
            token.with_value(TypeKeyword::Type),
        ref token @ Token{value: TokenKind::Identifier(ref id), ..} if id == "Ty" =>
            token.with_value(TypeKeyword::Ty),
    }
}

fn keyword_set<'tokens, I>()
-> impl Parser<'tokens, I, Token<pir::command::keyword::SetKeyword>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::command::keyword::SetKeyword;
    select! {
        ref token @ Token{value: TokenKind::Identifier(ref id), ..} if id == "Set" =>
                token.with_value(SetKeyword),
    }
}

fn keyword_unset<'tokens, I>()
-> impl Parser<'tokens, I, Token<pir::command::keyword::UnsetKeyword>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::command::keyword::UnsetKeyword;
    select! {
        ref token @ Token{value: TokenKind::Identifier(ref id), ..} if id == "Unset" =>
                token.with_value(UnsetKeyword),
    }
}
/// parse command
/// ```ebfn
/// command_kind :=
/// | keyword_definition expr_definition
/// | keyword_type type_definition
/// | keyword_eval expr
/// | keyword_typeof expr
/// | keyword_set variable
/// | keyword_unset variable
///```
pub fn command_kind<'tokens, I>() -> impl Parser<'tokens, I, pir::CommandKind, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::command::CommandKind;

    choice((
        keyword_definition()
            .then(expression_definition().map(Box::new))
            .map(|(keyword, def)| CommandKind::ExpressionDefinition { keyword, def }),
        keyword_type()
            .then(type_definition())
            .map(|(keyword, def)| CommandKind::TypeDefinition { keyword, def }),
        keyword_eval()
            .then(expression())
            .map(|(keyword, expr)| CommandKind::Eval { keyword, expr }),
        keyword_type_of()
            .then(expression())
            .map(|(keyword, expr)| CommandKind::TypeOf { keyword, expr }),
        keyword_help()
            .then(help_variable())
            .map(|(keyword, var)| CommandKind::Help { keyword, var }),
        keyword_set()
            .then(variable())
            .map(|(keyword, var)| CommandKind::Set { keyword, var }),
        keyword_unset()
            .then(variable())
            .map(|(keyword, var)| CommandKind::UnSet { keyword, var }),
    ))
}

/// parse command with dot
/// ```ebfn
/// command := command_kind "."
///```
pub fn command<'tokens, I>() -> impl Parser<'tokens, I, pir::Command, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    let dot = operator(Operator::Dot, pir::operator::Dot).labelled(".");
    command_kind()
        .then(dot)
        .map(|(kind, dot)| pir::Command { kind, dot })
        .labelled("command")
}

// ===========================================================================
// End of input
// ===========================================================================

/// parse end of input
pub fn end_of_input<'tokens, I>()
-> impl Parser<'tokens, I, Token<pir::file::EndOfInput>, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    use pir::file::EndOfInput;
    select! {token @ Token{ value: TokenKind::EndOfInput, ..} =>
        token.with_value(EndOfInput)
    }
    .labelled("")
}

// ===========================================================================
// File
// ===========================================================================

/// parse a whole file; a broken command is skipped up to the next `.` (its
/// error is recorded) and parsing goes on with the next command
/// ```ebfn
/// file := command* END_OF_INPUT
///```
pub fn file<'tokens, I>() -> impl Parser<'tokens, I, pir::File, Extra<'tokens>>
where
    I: ValueInput<'tokens, Token = Token<TokenKind>, Span = SimpleSpan>,
{
    let dot = operator(Operator::Dot, pir::operator::Dot).ignored();
    // a command is only tried before the end of input, so the recovery always
    // skips something
    let command = end_of_input()
        .not()
        .ignore_then(command().map(Some).recover_with(skip_until(
            any().ignored(),
            dot.or(end_of_input().ignored().rewind()),
            || None,
        )));
    command
        .repeated()
        .collect::<Vec<_>>()
        .then(end_of_input())
        .map(|(commands, end)| pir::File {
            commands: commands.into_iter().flatten().collect(),
            end: Some(end),
        })
}
