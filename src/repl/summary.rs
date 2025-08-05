use crate::typing::ast::{Definition, Typed};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct SummaryDefinition<'a>(&'a Definition);

impl<'a> From<&'a Definition> for SummaryDefinition<'a> {
    fn from(def: &'a Definition) -> Self {
        Self(def)
    }
}

impl Pretty for SummaryDefinition<'_> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let def = &self.0.value;
        Doc::nil()
            .append(theme.def_var(&def.name().name()))
            .append(Doc::space())
            .append(theme.operator(&":"))
            .append(Doc::space())
            .append(def.ty().pretty(theme))
    }
}
