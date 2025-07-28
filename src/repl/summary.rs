use crate::typing::ast::{ExpressionDefinition, Typed};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct SummaryDefinition<'a>(&'a ExpressionDefinition);

impl<'a> From<&'a ExpressionDefinition> for SummaryDefinition<'a> {
    fn from(def: &'a ExpressionDefinition) -> Self {
        Self(def)
    }
}

impl Pretty for SummaryDefinition<'_> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(theme.def_var(&self.0.name().name()))
            .append(Doc::space())
            .append(theme.operator(&":"))
            .append(Doc::space())
            .append(self.0.ty().pretty(theme))
    }
}
