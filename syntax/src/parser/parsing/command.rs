use super::*;
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
            .then(variable())
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
