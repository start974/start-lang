use super::super::{Expression, Type};
use super::rule_name::RuleName;
use super::syntax::Syntax;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub enum Command<T> {
    Add {
        syntax: Syntax,
        rule_name: RuleName,
        template: T,
    },
    Rm {
        rule_name: RuleName,
    },
}

pub type CommandExpression = Command<Expression>;
pub type CommandType = Command<Type>;

trait CommandName {
    fn name(&self) -> &'static str;
}

impl CommandName for CommandExpression {
    fn name(&self) -> &'static str {
        "Expression"
    }
}

impl CommandName for CommandType {
    fn name(&self) -> &'static str {
        "Type"
    }
}

impl<T> Pretty for Command<T>
where
    T: Pretty,
    Self: CommandName,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Command::Add {
                syntax,
                rule_name,
                template,
            } => Doc::nil()
                .append(theme.keyword(&"Add"))
                .append(Doc::space())
                .append(self.name())
                .append(Doc::space())
                .append(Doc::group(syntax.pretty(theme)))
                .append(Doc::space())
                .append(Doc::group(
                    Doc::nil()
                        .append(theme.op_typed_by())
                        .append(Doc::space())
                        .append(rule_name.pretty(theme)),
                ))
                .append(Doc::space())
                .append(theme.op_eq_def())
                .append(Doc::line())
                .append(Doc::group(template.pretty(theme))),
            Command::Rm { rule_name } => Doc::nil()
                .append(theme.keyword(&"Rm"))
                .append(Doc::space())
                .append(rule_name.pretty(theme)),
        }
    }
}

pub enum Commands {
    Expression(CommandExpression),
    Type(CommandType),
}

impl Pretty for Commands {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Commands::Expression(cmd) => cmd.pretty(theme),
            Commands::Type(cmd) => cmd.pretty(theme),
        }
    }
}
