use chumsky::input::ValueInput;
use chumsky::prelude::*;
use ir::pir::*;
use ir::token::{MetaToken, Operator, Token};

use crate::parser::CommandOrEnd;
use ir::pir;

pub type ErrorChumsky<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, MetaToken>>;

// ===========================================================================
// Operator
// ===========================================================================
pub fn operator<'tokens, I, Op>(
    op: Operator,
    res: Op,
) -> impl Parser<'tokens, I, Meta<Op>, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
    Op: Clone,
{
    select! {ref meta @ Meta{ value: Token::Operator(ref token_op), ..} if token_op == &op =>
            meta.clone().map(|_| res.clone())
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
pub fn pattern<'tokens, I>() -> impl Parser<'tokens, I, pir::Pattern, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
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
pub fn constant<'tokens, I>() -> impl Parser<'tokens, I, pir::Constant, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::expression::constant::{BuiltinT, Constant};
    let number = select! {ref meta @ Meta{ value: Token::Number(ref n), ..} =>
            meta.clone().map(|_| n.clone())
    }
    .map(Constant::from);

    let builtin = select! {
        ref meta @ Meta{ value: Token::Identifier(ref b), ..} if b == "__Constant_true__" =>
            meta.clone().map(|_| BuiltinT::True),
        ref meta @ Meta{ value: Token::Identifier(ref b), ..} if b == "__Constant_false__" =>
            meta.clone().map(|_| BuiltinT::False)
    }
    .map(Constant::from);

    let character = select! {ref meta @ Meta{ value: Token::Character(ref c), ..} =>
        meta.clone().map(|_| *c)
    }
    .map(Constant::from);

    choice((builtin, number, character)).labelled("constant")
}

/// parse variable
/// ```ebfn
/// variable := IDENTIFIER
///```
pub fn variable<'tokens, I>() -> impl Parser<'tokens, I, pir::Variable, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    select! {ref meta @ Meta{ value: Token::Identifier(ref s), ..} =>
            meta.clone().map(|_| s.clone())
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
    expr: impl Parser<'tokens, I, pir::Expression, ErrorChumsky<'tokens>>,
) -> impl Parser<'tokens, I, pir::Expression, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::Expression;
    use pir::expression::parenthesis::Paren;

    let constant = constant().map(Expression::Constant);
    let variable = variable().map(Expression::Variable);
    let parens = {
        let l_paren = operator(Operator::LParen, pir::operator::LParenT()).labelled("(");
        let r_paren = operator(Operator::RParen, pir::operator::RParenT()).labelled(")");
        l_paren
            .then(expr.map(Box::new))
            .then(r_paren)
            .map(|((l_paren, expr), r_paren)| Paren::new(l_paren, expr, r_paren))
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
    expr0: impl Parser<'tokens, I, pir::Expression, ErrorChumsky<'tokens>> + Clone,
) -> impl Parser<'tokens, I, pir::Expression, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let colon = operator(Operator::Colon, pir::operator::ColonT()).labelled(":");
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
pub fn expression<'tokens, I>() -> impl Parser<'tokens, I, pir::Expression, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
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
-> impl Parser<'tokens, I, pir::ExpressionDefinition, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::expression::definition::{ExpressionDefinition, TypedBy};

    let typed_by = {
        let colon = operator(Operator::Colon, pir::operator::ColonT()).labelled(":");
        colon.then(ty()).map(|(colon, ty)| TypedBy { colon, ty })
    };
    let eq_def = operator(Operator::EqDef, pir::operator::EqDefT()).labelled(":=");
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
pub fn ty_variable<'tokens, I>() -> impl Parser<'tokens, I, pir::Variable, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    variable().labelled("type variable")
}

/// parse type builtin
pub fn ty_builtin<'tokens, I>() -> impl Parser<'tokens, I, pir::ty::Builtin, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    select! {
        ref meta @ Meta{ value: Token::Identifier(ref s), ..} if s == "__Type_Nat__" =>
            meta.clone().map(|_| pir::ty::BuiltinT::Nat),

        ref meta @ Meta{ value: Token::Identifier(ref s), ..} if s == "__Type_Bool__" =>
            meta.clone().map(|_| pir::ty::BuiltinT::Bool),

        ref meta @ Meta{ value: Token::Identifier(ref s), ..} if s == "__Type_Char__" =>
            meta.clone().map(|_| pir::ty::BuiltinT::Char),
    }
    .labelled("builtin variable")
}

/// parse type
/// ```ebfn
/// type :=
/// | type_variable
/// ```
pub fn ty<'tokens, I>() -> impl Parser<'tokens, I, pir::Type, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let builtin = ty_builtin().map(pir::Type::Builtin);
    let var = ty_variable().map(pir::Type::Variable);

    choice((builtin, var)).labelled("type")
}

/// parse type definition
/// ```ebfn
/// type_definition := type_variable EQ_DEF type
/// ```
pub fn type_definition<'tokens, I>()
-> impl Parser<'tokens, I, pir::TypeDefinition, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let eq_def = operator(Operator::EqDef, pir::operator::EqDefT()).labelled(":=");
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
pub fn help_variable<'tokens, I>() -> impl Parser<'tokens, I, pir::Variable, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    variable().labelled("help variable")
}
// ===========================================================================
// Command
// ===========================================================================

fn keyword_definition<'tokens, I>()
-> impl Parser<'tokens, I, pir::command::keyword::DefinitionKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::command::keyword::DefinitionKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Def" =>
            meta.clone().map(|_| DefinitionKeywordT::Def),
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Definition" =>
            meta.clone().map(|_| DefinitionKeywordT::Definition),
    }
}

fn keyword_eval<'tokens, I>()
-> impl Parser<'tokens, I, pir::command::keyword::EvalKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::command::keyword::EvalKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Eval" =>
            meta.clone().map(|_| EvalKeywordT::Eval),
        ref meta @ Meta{value: Token::Operator(Operator::Eval), ..} =>
            meta.clone().map(|_| EvalKeywordT::EvalOp)
    }
}

fn keyword_type_of<'tokens, I>()
-> impl Parser<'tokens, I, pir::command::keyword::TypeOfKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::command::keyword::TypeOfKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "TypeOf" =>
            meta.clone().map(|_| TypeOfKeywordT::TypeOf),
        ref meta @ Meta{value: Token::Operator(Operator::TypeOf), ..} =>
            meta.clone().map(|_| TypeOfKeywordT::TypeOfOp),
    }
}

fn keyword_help<'tokens, I>()
-> impl Parser<'tokens, I, pir::command::keyword::HelpKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::command::keyword::HelpKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Help" =>
            meta.clone().map(|_| HelpKeywordT::Help),
        ref meta @ Meta{value: Token::Operator(Operator::Help), ..} =>
            meta.clone().map(|_| HelpKeywordT::HelpOp),
    }
}

fn keyword_type<'tokens, I>()
-> impl Parser<'tokens, I, pir::command::keyword::TypeKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::command::keyword::TypeKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Type" =>
            meta.clone().map(|_| TypeKeywordT::Type),
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Ty" =>
            meta.clone().map(|_| TypeKeywordT::Ty),
    }
}

fn keyword_set<'tokens, I>()
-> impl Parser<'tokens, I, pir::command::keyword::SetKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::command::keyword::SetKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Set" =>
                meta.clone().map(|_| SetKeywordT()),
    }
}

fn keyword_unset<'tokens, I>()
-> impl Parser<'tokens, I, pir::command::keyword::UnsetKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::command::keyword::UnsetKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Unset" =>
                meta.clone().map(|_| UnsetKeywordT()),
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
pub fn command_kind<'tokens, I>() -> impl Parser<'tokens, I, pir::CommandKind, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
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
pub fn command<'tokens, I>() -> impl Parser<'tokens, I, pir::Command, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let dot = operator(Operator::Dot, pir::operator::DotT()).labelled(".");
    command_kind()
        .then(dot)
        .map(|(kind, dot)| pir::Command { kind, dot })
        .labelled("command")
}

// ===========================================================================
// End of input
// ===========================================================================

/// parse end of input
pub fn end_of_input<'tokens, I>() -> impl Parser<'tokens, I, pir::EndOfFile, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use pir::file::EndOfFileT;
    select! {meta @ Meta{ value: Token::EndOfInput, ..} =>
        meta.map(|_| EndOfFileT())
    }
    .labelled("")
}

// ===========================================================================
// End of input
// ===========================================================================

/// parse with lexer tokens
pub fn parser<'tokens, I>() -> impl Parser<'tokens, I, CommandOrEnd, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let command = command().map(Box::new).map(CommandOrEnd::Command);
    let eoi = end_of_input().map(CommandOrEnd::End);
    choice((command, eoi))
}
