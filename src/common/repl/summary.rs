use crate::typer::ast::{ExpressionDefinition, Pattern, Typed};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct SummaryDefinition<'a>(&'a ExpressionDefinition);

impl<'a> From<&'a ExpressionDefinition> for SummaryDefinition<'a> {
    fn from(def: &'a ExpressionDefinition) -> Self {
        Self(def)
    }
}

fn pretty_pattern<'a>(pat: &Pattern, theme: &Theme) -> Doc<'a> {
    match pat {
        Pattern::Variable(var) => theme.def_var(&var.identifier().name()),
    }
}

impl Pretty for SummaryDefinition<'_> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let def = self.0;
        Doc::nil()
            .append(pretty_pattern(self.0.pattern(), theme))
            .append(Doc::space())
            .append(theme.operator(&":"))
            .append(Doc::space())
            .append(def.ty().pretty(theme))
    }
}
