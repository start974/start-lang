use super::Expression;
use super::ExpressionDefinition;
use super::TypeDefinition;

use crate::utils::pretty::Pretty;
use crate::utils::theme::Doc;
use crate::utils::theme::Theme;

pub enum Command {
    ExpressionDefinition(ExpressionDefinition),
    TypeDefinition(TypeDefinition),
    Eval(Expression),
    //Grammar(Grammar),
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
            //Command::Grammar(grammar) => grammar.pretty(theme),
            Command::TypeOf(expr) => Doc::nil()
                .append(theme.keyword(&"TypeOf"))
                .append(Doc::space())
                .append(expr.pretty(theme)),
        }
    }
}
