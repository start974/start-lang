use super::{grammar, Expression, ExpressionDefinition, TypeDefinition};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub enum Command {
    ExpressionDefinition(ExpressionDefinition),
    TypeDefinition(TypeDefinition),
    Eval(Expression),
    Grammar(grammar::command::Commands),
    TypeOf(Expression),
}

impl Pretty for Command {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Command::ExpressionDefinition(def) => Doc::nil()
                .append(theme.keyword(&"Definition"))
                .append(Doc::space())
                .append(def.pretty(theme)),
            Command::TypeDefinition(def) => Doc::nil()
                .append(theme.keyword(&"Type"))
                .append(Doc::space())
                .append(def.pretty(theme)),
            Command::Eval(expr) => Doc::nil()
                .append(theme.keyword(&"Eval"))
                .append(Doc::space())
                .append(expr.pretty(theme)),
            Command::Grammar(grammar_cmd) => Doc::nil()
                .append(theme.keyword(&"Grammar"))
                .append(Doc::space())
                .append(grammar_cmd.pretty(theme)),
            Command::TypeOf(expr) => Doc::nil()
                .append(theme.keyword(&"TypeOf"))
                .append(Doc::space())
                .append(expr.pretty(theme)),
        }
    }
}
