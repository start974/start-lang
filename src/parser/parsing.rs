use super::cst;
use super::cst::WithComments;
use super::ErrorChumsky;
use crate::lexer::token::Operator;
use crate::lexer::token::Token;
use crate::utils::location::Location;
use crate::utils::location::SourceId;
use chumsky::input::ValueInput;
use chumsky::prelude::*;

// ===========================================================================
// with comments
// ===========================================================================

pub trait WithCommentsExt<'tokens, I, O>:
    Parser<'tokens, I, O, ErrorChumsky<'tokens>> + Sized
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
    O: WithComments,
{
    /// with comment parser
    fn with_comments(self) -> impl Parser<'tokens, I, O, ErrorChumsky<'tokens>> {
        let comments = select! {Token::Comment(c) => c}
            .map(cst::Comment::from)
            .repeated()
            .collect();

        comments
            .then(self)
            .then(comments)
            .map(|((comments_before, value), comments_after)| {
                value
                    .with_comments_before(comments_before)
                    .with_comments_after(comments_after)
            })
    }
}

impl<'tokens, I, O, P> WithCommentsExt<'tokens, I, O> for P
where
    P: Parser<'tokens, I, O, ErrorChumsky<'tokens>> + Sized,
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
    O: WithComments,
{
}

// ===========================================================================
// Identifier
// ===========================================================================
/// parse identifier
/// ```ebfn
/// identifier := IDENTIFIER
/// ```
pub fn identifier<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::Identifier<String>, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    select! {Token::Identifier(id) => id}
        .map_with(move |id, e| {
            let span: SimpleSpan = e.span();
            let loc = Location::new(source_id.clone(), span.start, span.end);
            cst::Identifier::new(id, loc)
        })
        .with_comments()
}

// ===========================================================================
// Pattern
// ===========================================================================
/// parse pattern
/// ```ebfn
/// constant :=
/// | IDENTIFIER
///```
pub fn pattern<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::Pattern, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    let variable = identifier(source_id)
        .map(|id| id.map_name(cst::PatternVariableName::from))
        .map(cst::Pattern::from)
        .labelled("pattern variable");

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
pub fn constant<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::Constant, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    let number = {
        select! {Token::Number(n) => n}.map_with({
            let source_id = source_id.clone();
            move |n, e| {
                let span: SimpleSpan = e.span();
                let loc = Location::new(source_id.clone(), span.start, span.end);
                cst::Constant::nat(n, loc)
            }
        })
    };

    let character = select! {Token::Character(c) => c}.map_with(move |c, e| {
        let span: SimpleSpan = e.span();
        let loc = Location::new(source_id.clone(), span.start, span.end);
        cst::Constant::char(c, loc)
    });

    choice((number, character))
        .with_comments()
        .labelled("constant")
}

/// parse variable
/// ```ebfn
/// variable := IDENTIFIER
///```
pub fn variable<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::Variable, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    identifier(source_id)
        .map(|id| id.map_name(cst::VariableName::from))
        .map(cst::Variable::from)
        .labelled("variable")
}

/// parse expression
/// ```ebfn
/// expr0 :=
/// | "(" expr@1 ")"
/// | variable
/// | constant
///
/// expr@1 :=
/// | expr@0 COLON ty
/// | expr@0
///```
pub fn expression<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::Expression, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    recursive(move |expr1| {
        let expr0 = {
            let variable = variable(source_id.clone()).map(cst::Expression::from);
            let constant = constant(source_id.clone()).map(cst::Expression::from);
            let parens = expr1.delimited_by(
                just(Token::Operator(Operator::LParen)).labelled("("),
                just(Token::Operator(Operator::RParen)).labelled(")"),
            );
            choice((variable, constant, parens))
        }
        .boxed();

        let expr1 = {
            let type_restriction = (expr0.clone())
                .then_ignore(just(Token::Operator(Operator::Colon)).labelled("Colon"))
                .then(ty(source_id.clone()))
                .map(|(expr, ty)| cst::TypeRestriction::new(expr, ty))
                .map(cst::Expression::from);
            choice((type_restriction, expr0))
        }
        .boxed();

        expr1
    })
    .labelled("expression")
}

/// parse expression definition
/// ```ebfn
/// expr_definition := pattern (COLON type)? EQ_DEF expression
///```
pub fn expression_definition<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::ExpressionDefinition, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    pattern(source_id.clone())
        .then(
            (just(Token::Operator(Operator::Colon)).labelled(":"))
                .ignore_then(ty(source_id.clone()))
                .or_not(),
        )
        .then_ignore(just(Token::Operator(Operator::EqDef)).labelled(":="))
        .then(expression(source_id))
        .map(|((id, opt_ty), body)| {
            let def = cst::ExpressionDefinition::new(id, body);
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
/// type_variable := IDENTIFIER
/// ```
pub fn ty_variable<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::TypeVariable, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    identifier(source_id)
        .map(|id| id.map_name(cst::TypeVariableName::from))
        .labelled("type variable")
}

/// parse type
/// ```ebfn
/// type :=
/// | type_variable
/// ```
pub fn ty<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::Type, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    ty_variable(source_id).map(cst::Type::from).labelled("type")
}

/// parse type definition
/// ```ebfn
/// type_definition := type_variable EQ_DEF type
/// ```
pub fn type_definition<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::TypeDefinition, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    ty_variable(source_id.clone())
        .then_ignore(just(Token::Operator(Operator::EqDef)).labelled(":="))
        .then(ty(source_id))
        .map(|(name, ty)| cst::TypeDefinition::new(name, ty))
}

// ===========================================================================
// Command
// ===========================================================================

/// parse command
/// ```ebfn
/// command_kind :=
/// | ("Definition" | "Def") expr_definition
/// | ("Eval" | EVAL_OP) expr
/// | ("TypeOf" | TYPE_OF_OP) expr
/// | ("Type" | "Ty") type_definition
/// | ("Set") | "Unset") variable
///```
pub fn command_kind<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::CommandKind, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    let def_expr = select! {Token::Identifier(id) if id == "Def" || id == "Definition" => ()}
        .labelled("Definition")
        .ignore_then(expression_definition(source_id.clone()))
        .map(cst::CommandKind::ExpressionDefinition);

    let eval = select! {
        Token::Identifier(id) if id == "Eval" => (),
        Token::Operator(Operator::Eval) => ()
    }
    .labelled("Eval")
    .ignore_then(expression(source_id.clone()))
    .map(cst::CommandKind::Eval);

    let type_of = select! {
        Token::Identifier(id) if id == "TypeOf"  => (),
        Token::Operator(Operator::TypeOf) => ()
    }
    .labelled("TypeOf")
    .ignore_then(expression(source_id.clone()))
    .map(cst::CommandKind::TypeOf);

    let def_type = select! {Token::Identifier(id) if id == "Ty" || id == "Type" => ()}
        .labelled("Type")
        .ignore_then(type_definition(source_id.clone()))
        .map(cst::CommandKind::TypeDefinition);

    let set_unset = choice((
        select! {
            Token::Identifier(id) if id == "Set" => true
        }
        .labelled("Set"),
        select! {
        Token::Identifier(id) if id == "Unset" => false
        }
        .labelled("Unset"),
    ))
    .then(variable(source_id.clone()))
    .map(|(b, var)| cst::CommandKind::Set(b, var));

    choice((def_expr, eval, type_of, def_type, set_unset))
}

/// parse command with dot
/// ```ebfn
/// command := command_kind "."
///```
pub fn command<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, cst::Command, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    command_kind(source_id.clone())
        .then_ignore(just(Token::CommandEnd))
        .labelled("command")
        .map_with(move |cmd_kind, e| {
            let span: SimpleSpan = e.span();
            let loc = Location::new(source_id.clone(), span.start, span.end);
            cst::Command::new(cmd_kind, loc)
        })
}
