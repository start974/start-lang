use super::{Documentation, Identifier, Type};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub enum HelpInfo {
    Expression(Type),
    Alias(Type),
}
// ==========================================================================
// Help Variable
// ==========================================================================

pub struct Help {
    pub var: Identifier,
    pub info: HelpInfo,
    pub doc: Option<Documentation>,
}

impl Pretty for Help {
    fn pretty(&self, theme: &Theme) -> Doc {
        let var = match &self.info {
            HelpInfo::Expression(ty) => Doc::nil()
                .append(theme.expr_var(&self.var.name()))
                .append(Doc::space())
                .append(theme.operator(&":"))
                .append(Doc::softline())
                .append(ty.pretty(theme).group()),
            HelpInfo::Alias(Type::Builtin(_)) => Doc::nil(),
            HelpInfo::Alias(ty) => Doc::nil()
                .append(theme.ty_var(&self.var.name()))
                .append(Doc::space())
                .append(theme.operator(&":="))
                .append(Doc::softline())
                .append(ty.pretty(theme).group()),
        };

        let docu = match &self.doc {
            Some(doc) => Doc::hardline().append(doc.pretty(theme)).nest(2),
            None => Doc::nil(),
        };

        var.append(docu)
    }
}
