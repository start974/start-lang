use location::Span;

use super::CommandOrEnd;

mod expression;
mod operator;
mod pattern;

pub use crate::lexer::token::{Operator, Token, TokenKind, TokenStream};
pub use chumsky::prelude::*;
use cst::Meta;
pub use expression::{constant, expression, expression_definition, variable};
pub use operator::operator;
pub use pattern::pattern;

pub type ErrorChumsky = chumsky::error::Rich<'static, Token, Span>;
pub type ExtraChumsky = chumsky::extra::Err<ErrorChumsky>;

// ===========================================================================
// Type
// ===========================================================================
/// parse type
/// ```ebfn
/// type_variable := IDENTIFIER
/// ```
pub fn ty_variable() -> impl Parser<'static, TokenStream, cst::ty::Variable, ExtraChumsky> {
    select! {ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} =>
            meta.clone().map(|_| cst::ty::VariableT::from(s.clone()))
    }
    .labelled("type variable")
}

/// parse type builtin
pub fn ty_builtin() -> impl Parser<'static, TokenStream, cst::ty::Builtin, ExtraChumsky> {
    select! {
        ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} if s == "__Type_Nat__" =>
            meta.clone().map(|_| cst::ty::BuiltinT::Nat),

        ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} if s == "__Type_Bool__" =>
            meta.clone().map(|_| cst::ty::BuiltinT::Bool),

        ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} if s == "__Type_Char__" =>
            meta.clone().map(|_| cst::ty::BuiltinT::Char),
    }
    .labelled("builtin variable")
}

/// parse type
/// ```ebfn
/// type :=
/// | type_variable
/// ```
pub fn ty() -> impl Parser<'static, TokenStream, cst::Type, ExtraChumsky> {
    let builtin = ty_builtin().map(cst::Type::Builtin);
    let var = ty_variable().map(cst::Type::Variable);

    choice((builtin, var)).labelled("type")
}

/// parse type definition
/// ```ebfn
/// type_definition := type_variable EQ_DEF type
/// ```
pub fn type_definition() -> impl Parser<'static, TokenStream, cst::TypeDefinition, ExtraChumsky> {
    let eq_def = operator(Operator::EqDef, cst::operator::EqDefT()).labelled(":=");
    ty_variable()
        .then(eq_def)
        .then(ty())
        .map(|((name, eq_def), ty)| cst::TypeDefinition { name, eq_def, ty })
}

// ===========================================================================
// Type
// ===========================================================================
/// parse type
/// ```ebfn
/// help_variable := IDENTIFIER
/// ```
pub fn help_variable() -> impl Parser<'static, TokenStream, cst::help::Variable, ExtraChumsky> {
    use cst::help::VariableT;
    select! {ref meta @ Meta{ value: TokenKind::Identifier(ref s), ..} =>
            meta.clone().map(|_| VariableT::from(s.clone()))
    }
    .labelled("help variable")
}
// ===========================================================================
// Command
// ===========================================================================

fn keyword_definition()
-> impl Parser<'static, TokenStream, cst::command::DefinitionKeyword, ExtraChumsky> {
    use cst::command::DefinitionKeywordT;
    select! {
        ref meta @ Meta{value: TokenKind::Identifier(ref id), ..} if id == "Def" =>
            meta.clone().map(|_| DefinitionKeywordT::Def),
        ref meta @ Meta{value: TokenKind::Identifier(ref id), ..} if id == "Definition" =>
            meta.clone().map(|_| DefinitionKeywordT::Definition),
    }
}

fn keyword_eval() -> impl Parser<'static, TokenStream, cst::command::EvalKeyword, ExtraChumsky> {
    use cst::command::EvalKeywordT;
    select! {
        ref meta @ Meta{value: TokenKind::Identifier(ref id), ..} if id == "Eval" =>
            meta.clone().map(|_| EvalKeywordT::Eval),
        ref meta @ Meta{value: TokenKind::Operator(Operator::Eval), ..} =>
            meta.clone().map(|_| EvalKeywordT::EvalOp)
    }
}

fn keyword_type_of() -> impl Parser<'static, TokenStream, cst::command::TypeOfKeyword, ExtraChumsky>
{
    use cst::command::TypeOfKeywordT;
    select! {
        ref meta @ Meta{value: TokenKind::Identifier(ref id), ..} if id == "TypeOf" =>
            meta.clone().map(|_| TypeOfKeywordT::TypeOf),
        ref meta @ Meta{value: TokenKind::Operator(Operator::TypeOf), ..} =>
            meta.clone().map(|_| TypeOfKeywordT::TypeOfOp),
    }
}

fn keyword_help() -> impl Parser<'static, TokenStream, cst::command::HelpKeyword, ExtraChumsky> {
    use cst::command::HelpKeywordT;
    select! {
        ref meta @ Meta{value: TokenKind::Identifier(ref id), ..} if id == "Help" =>
            meta.clone().map(|_| HelpKeywordT::Help),
        ref meta @ Meta{value: TokenKind::Operator(Operator::Help), ..} =>
            meta.clone().map(|_| HelpKeywordT::HelpOp),
    }
}

fn keyword_type() -> impl Parser<'static, TokenStream, cst::command::TypeKeyword, ExtraChumsky> {
    use cst::command::TypeKeywordT;
    select! {
        ref meta @ Meta{value: TokenKind::Identifier(ref id), ..} if id == "Type" =>
            meta.clone().map(|_| TypeKeywordT::Type),
        ref meta @ Meta{value: TokenKind::Identifier(ref id), ..} if id == "Ty" =>
            meta.clone().map(|_| TypeKeywordT::Ty),
    }
}

fn keyword_set() -> impl Parser<'static, TokenStream, cst::command::SetKeyword, ExtraChumsky> {
    use cst::command::SetKeywordT;
    select! {
        ref meta @ Meta{value: TokenKind::Identifier(ref id), ..} if id == "Set" =>
                meta.clone().map(|_| SetKeywordT()),
    }
}

fn keyword_unset() -> impl Parser<'static, TokenStream, cst::command::UnsetKeyword, ExtraChumsky> {
    use cst::command::UnsetKeywordT;
    select! {
        ref meta @ Meta{value: TokenKind::Identifier(ref id), ..} if id == "Unset" =>
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
pub fn command_kind() -> impl Parser<'static, TokenStream, cst::CommandKind, ExtraChumsky> {
    use cst::command::CommandKind;

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
pub fn command() -> impl Parser<'static, TokenStream, cst::Command, ExtraChumsky> {
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
pub fn end_of_input() -> impl Parser<'static, TokenStream, cst::EndOfFile, ExtraChumsky> {
    use cst::file::EndOfFileT;
    select! {meta @ Meta{ value: TokenKind::EndOfInput, ..} =>
        meta.map(|_| EndOfFileT())
    }
    .labelled("")
}

// ===========================================================================
// End of input
// ===========================================================================

/// parse with lexer tokens
pub fn parser() -> impl Parser<'static, TokenStream, CommandOrEnd, ExtraChumsky> {
    let command = command().map(Box::new).map(CommandOrEnd::Command);
    let eoi = end_of_input().map(CommandOrEnd::End);
    choice((command, eoi))
}
