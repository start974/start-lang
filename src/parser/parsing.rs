use super::ast;
use super::ErrorChumsky;
use crate::lexer::token::Keyword;
use crate::lexer::token::Operator;
use crate::lexer::token::Token;
use crate::utils::location::Location;
use crate::utils::location::SourceId;
use chumsky::input::ValueInput;
use chumsky::prelude::*;

// ===========================================================================
// Identifier
// ===========================================================================
/// parse identifier
/// ```ebfn
/// identifier := IDENTIFIER
/// ```
pub fn identifier<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, ast::Identifier, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    select! {Token::Identifier(id) => id}
        .map_with(move |id, e| {
            let span: SimpleSpan = e.span();
            let loc = Location::new(source_id.clone(), span.start, span.end);
            ast::Identifier::new(&id, loc)
        })
        .labelled("identifier")
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
) -> impl Parser<'tokens, I, ast::Constant, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    let number = {
        select! {Token::Number(n) => n}.map_with({
            let source_id = source_id.clone();
            move |n, e| {
                let span: SimpleSpan = e.span();
                let loc = Location::new(source_id.clone(), span.start, span.end);
                ast::Constant::nat(n, loc)
            }
        })
    };

    let character = select! {Token::Character(c) => c}.map_with(move |c, e| {
        let span: SimpleSpan = e.span();
        let loc = Location::new(source_id.clone(), span.start, span.end);
        ast::Constant::char(c, loc)
    });

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
/// | expr@0 COLON ty
/// | expr@0
///```
pub fn expression<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, ast::Expression, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    recursive(move |expr1| {
        let expr0 = {
            let identifier = identifier(source_id.clone()).map(ast::Expression::from);
            let constant = constant(source_id.clone()).map(ast::Expression::from);
            let parens = expr1.delimited_by(
                just(Token::Operator(Operator::LParen)),
                just(Token::Operator(Operator::RParen)),
            );
            choice((identifier, constant, parens)).boxed()
        };

        let expr1 = {
            let type_restriction = (expr0.clone())
                .then_ignore(just(Token::Operator(Operator::Colon)))
                .then(ty(source_id))
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
/// expr_definition := identifier (COLON type)? EQ_DEF expression
///```
pub fn expression_definition<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, ast::ExpressionDefinition, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    identifier(source_id.clone())
        .then(
            just(Token::Operator(Operator::Colon))
                .ignore_then(ty(source_id.clone()))
                .or_not(),
        )
        .then_ignore(just(Token::Operator(Operator::EqDef)))
        .then(expression(source_id))
        .map(|((id, opt_ty), body)| {
            let def = ast::ExpressionDefinition::new(id, body);
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
pub fn ty<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, ast::Type, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    identifier(source_id).map(ast::Type::from).labelled("type")
}

/// parse type definition
/// ```ebfn
/// type_definition := identifier EQ_DEF type
/// ```
pub fn type_definition<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, ast::TypeDefinition, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    identifier(source_id.clone())
        .then_ignore(just(Token::Operator(Operator::EqDef)))
        .then(ty(source_id))
        .map(|(name, ty)| ast::TypeDefinition::new(name, ty))
}

// ===========================================================================
// Command
// ===========================================================================
/// parse command
/// ```ebfn
/// command_kind :=
/// | DEFINITY expr_definition
/// | TY type_definition
/// | EVAL expr
/// | TYPE_OF expr
/// | SET IDENTIFIER
/// | UNSET IDENTIFIER
///```
pub fn command_kind<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, ast::CommandKind, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    let def_expr = just(Token::Keyword(Keyword::Definition))
        .ignore_then(expression_definition(source_id.clone()))
        .map(ast::CommandKind::ExpressionDefinition);

    let def_type = just(Token::Keyword(Keyword::Type))
        .ignore_then(type_definition(source_id.clone()))
        .map(ast::CommandKind::TypeDefinition);

    let eval = just(Token::Keyword(Keyword::Eval))
        .ignore_then(expression(source_id.clone()))
        .map(ast::CommandKind::Eval);

    let type_of = just(Token::Keyword(Keyword::TypeOf))
        .ignore_then(expression(source_id.clone()))
        .map(ast::CommandKind::TypeOf);

    let set = (select! {Token::Keyword(Keyword::Set(b)) => b})
        .then(identifier(source_id.clone()))
        .map(|(b, id)| ast::CommandKind::Set(b, id));

    choice((def_expr, def_type, eval, type_of, set))
}

/// parse command with dot
/// ```ebfn
/// command := command_kind "."
///```
pub fn command<'tokens, I>(
    source_id: SourceId,
) -> impl Parser<'tokens, I, ast::Command, ErrorChumsky<'tokens>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    command_kind(source_id.clone())
        .then_ignore(just(Token::CommandEnd))
        .labelled("command")
        .map_with(move |cmd_kind, e| {
            let span: SimpleSpan = e.span();
            let loc = Location::new(source_id.clone(), span.start, span.end);
            ast::Command::new(cmd_kind, loc)
        })
}
