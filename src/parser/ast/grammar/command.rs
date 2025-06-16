use super::rule_name::RuleName;
use super::syntax::Syntax;
use super::template::Template;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub enum Command {
    Add {
        syntax: Syntax,
        rule_name: RuleName,
        template: Template,
    },
    Rm {
        rule_name: RuleName,
    },
}

impl Pretty for Command {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Command::Add {
                syntax,
                rule_name,
                template,
            } => Doc::nil()
                .append(theme.keyword(&"Add"))
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
