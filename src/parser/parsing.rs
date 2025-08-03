use super::cst;
use super::ErrorChumsky;
use crate::lexer::meta::Meta;
use crate::lexer::token::{MetaToken, Operator, Token};
use chumsky::input::ValueInput;
use chumsky::prelude::*;

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
pub fn pattern<'tokens, I>() -> impl Parser<'tokens, I, cst::Pattern, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let variable = select! {ref meta @ Meta{ value: Token::Identifier(ref s), ..} =>
            meta.clone().map(|_| cst::pattern::VariableT::from(s.clone()))
    }
    .labelled("pattern variable")
    .map(cst::Pattern::from);

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
pub fn constant<'tokens, I>() -> impl Parser<'tokens, I, cst::Constant, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let number = select! {ref meta @ Meta{ value: Token::Number(ref n), ..} =>
            meta.clone().map(|_| cst::constant::NumberT::from(n.clone()))
    }
    .map(cst::Constant::from);

    let character = select! {ref meta @ Meta{ value: Token::Character(ref c), ..} =>
    meta.clone().map(|_| cst::constant::CharacterT::from(*c))}
    .map(cst::Constant::from);

    choice((number, character)).labelled("constant")
}

/// parse variable
/// ```ebfn
/// variable := IDENTIFIER
///```
pub fn variable<'tokens, I>(
) -> impl Parser<'tokens, I, cst::expression::Variable, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::expression::VariableT;
    select! {ref meta @ Meta{ value: Token::Identifier(ref s), ..} =>
            meta.clone().map(|_| VariableT::from(s.clone()))
    }
}

/// parse expression0
/// ```ebfn
/// expression0 :=
/// | variable
/// | constant
/// | "(" expression ")"
///```
fn expression0<'tokens, I>(
    expr: impl Parser<'tokens, I, cst::Expression, ErrorChumsky<'tokens>>,
) -> impl Parser<'tokens, I, cst::expression::Expression0, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::expression::Expression0;
    use cst::parenthesis::Parenthesed;

    let variable = variable().map(Expression0::Variable);
    let constant = constant().map(Expression0::Constant);
    let parens = {
        let l_paren = operator(Operator::LParen, cst::operator::LParenT()).labelled("(");
        let r_paren = operator(Operator::RParen, cst::operator::RParenT()).labelled(")");
        l_paren
            .then(expr.map(Box::new))
            .then(r_paren)
            .map(|((l_paren, expr), r_paren)| Parenthesed::new(l_paren, expr, r_paren))
            .map(Expression0::Paren)
    };

    choice((variable, constant, parens))
}

/// parse expression1
/// ```ebfn
/// expr@1 :=
/// | expr@0 COLON ty
/// | expr@0
///```
fn expression1<'tokens, I>(
    expr0: impl Parser<'tokens, I, cst::expression::Expression0, ErrorChumsky<'tokens>> + Clone,
) -> impl Parser<'tokens, I, cst::expression::Expression1, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::expression::Expression1;

    let colon = operator(Operator::Colon, cst::operator::ColonT()).labelled(":");
    let type_restriction = (expr0.clone())
        .then(colon)
        .then(ty())
        .map(|((expr, colon), ty)| Expression1::TypedExpression { expr, colon, ty });

    let expr0_in1 = expr0.map(Expression1::Expression0);

    choice((type_restriction, expr0_in1))
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
pub fn expression<'tokens, I>() -> impl Parser<'tokens, I, cst::Expression, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    recursive(move |expr| {
        let expr0 = expression0(expr.clone()).boxed();
        let expr1 = expression1(expr0).boxed();
        expr1
    })
    .labelled("expression")
}

/// parse expression definition
/// ```ebfn
/// expr_definition := pattern (colon type)? EQ_DEF expression
///```
pub fn expression_definition<'tokens, I>(
) -> impl Parser<'tokens, I, cst::ExpressionDefinition, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::expression_definition::{ExpressionDefinition, TypedBy};

    let typed_by = {
        let colon = operator(Operator::Colon, cst::operator::ColonT()).labelled(":");
        colon.then(ty()).map(|(colon, ty)| TypedBy { colon, ty })
    };
    let eq_def = operator(Operator::EqDef, cst::operator::EqDefT()).labelled(":=");
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
pub fn ty_variable<'tokens, I>() -> impl Parser<'tokens, I, cst::ty::Variable, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    select! {ref meta @ Meta{ value: Token::Identifier(ref s), ..} =>
            meta.clone().map(|_| cst::ty::VariableT::from(s.clone()))
    }
    .labelled("type variable")
}

/// parse type
/// ```ebfn
/// type :=
/// | type_variable
/// ```
pub fn ty<'tokens, I>() -> impl Parser<'tokens, I, cst::Type, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let var = ty_variable().map(cst::Type::Variable);

    var.labelled("type")
}

/// parse type definition
/// ```ebfn
/// type_definition := type_variable EQ_DEF type
/// ```
pub fn type_definition<'tokens, I>(
) -> impl Parser<'tokens, I, cst::TypeDefinition, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let eq_def = operator(Operator::EqDef, cst::operator::EqDefT()).labelled(":=");
    ty_variable()
        .then(eq_def)
        .then(ty())
        .map(|((name, eq_def), ty)| cst::TypeDefinition { name, eq_def, ty })
}

// ===========================================================================
// Command
// ===========================================================================

fn keyword_definition<'tokens, I>(
) -> impl Parser<'tokens, I, cst::command::DefinitionKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::command::DefinitionKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Def" =>
            meta.clone().map(|_| DefinitionKeywordT::Def),
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Definition" =>
            meta.clone().map(|_| DefinitionKeywordT::Definition),
    }
}

fn keyword_eval<'tokens, I>(
) -> impl Parser<'tokens, I, cst::command::EvalKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::command::EvalKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Eval" =>
            meta.clone().map(|_| EvalKeywordT::Eval),
        ref meta @ Meta{value: Token::Operator(Operator::Eval), ..} =>
            meta.clone().map(|_| EvalKeywordT::EvalOp)
    }
}

fn keyword_type_of<'tokens, I>(
) -> impl Parser<'tokens, I, cst::command::TypeOfKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::command::TypeOfKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "TypeOf" =>
            meta.clone().map(|_| TypeOfKeywordT::TypeOf),
        ref meta @ Meta{value: Token::Operator(Operator::TypeOf), ..} =>
            meta.clone().map(|_| TypeOfKeywordT::TypeOfOp),
    }
}

fn keyword_type<'tokens, I>(
) -> impl Parser<'tokens, I, cst::command::TypeKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::command::TypeKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Type" =>
            meta.clone().map(|_| TypeKeywordT::Type),
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Ty" =>
            meta.clone().map(|_| TypeKeywordT::Ty),
    }
}

fn keyword_set<'tokens, I>(
) -> impl Parser<'tokens, I, cst::command::SetKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::command::SetKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Set" =>
                meta.clone().map(|_| SetKeywordT()),
    }
}

fn keyword_unset<'tokens, I>(
) -> impl Parser<'tokens, I, cst::command::UnsetKeyword, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::command::UnsetKeywordT;
    select! {
        ref meta @ Meta{value: Token::Identifier(ref id), ..} if id == "Unset" =>
                meta.clone().map(|_| UnsetKeywordT()),
    }
}
/// parse command
/// ```ebfn
/// command_kind :=
/// | keyword_definition expr_definition
/// | keyword_eval expr
/// | keyword_typeof expr
/// | keyword_type type_definition
/// | keyword_set variable
/// | keyword_unset variable
///```
pub fn command_kind<'tokens, I>() -> impl Parser<'tokens, I, cst::CommandKind, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::command::CommandKind;
    let def_expr = keyword_definition()
        .then(expression_definition().map(Box::new))
        .map(|(keyword, def)| CommandKind::ExpressionDefinition { keyword, def });

    let eval = keyword_eval()
        .then(expression())
        .map(|(keyword, expr)| CommandKind::Eval { keyword, expr });

    let type_of = keyword_type_of()
        .then(expression())
        .map(|(keyword, expr)| CommandKind::TypeOf { keyword, expr });

    let def_type = keyword_type()
        .then(type_definition())
        .map(|(keyword, def)| CommandKind::TypeDefinition { keyword, def });

    let set = keyword_set()
        .then(variable())
        .map(|(keyword, var)| CommandKind::Set { keyword, var });

    let unset = keyword_unset()
        .then(variable())
        .map(|(keyword, var)| CommandKind::UnSet { keyword, var });

    choice((def_expr, eval, type_of, def_type, set, unset))
}

/// parse command with dot
/// ```ebfn
/// command := command_kind "."
///```
pub fn command<'tokens, I>() -> impl Parser<'tokens, I, cst::Command, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    let dot = operator(Operator::Dot, cst::operator::DotT()).labelled(".");
    command_kind()
        .then(dot)
        .map(|(kind, dot)| cst::Command { kind, dot })
        .labelled("command")
}

// ===========================================================================
// End of input
// ===========================================================================

/// parse end of input
pub fn end_of_input<'tokens, I>() -> impl Parser<'tokens, I, cst::EndOfFile, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = MetaToken, Span = SimpleSpan>,
{
    use cst::file::EndOfFileT;
    select! {meta @ Meta{ value: Token::EndOfInput, ..} =>
        meta.map(|_| EndOfFileT())
    }.labelled("")
}
