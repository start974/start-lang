use super::{Documentation, Identifier, Type, Typed as _};
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub enum HelpInfo {
    Expression(Type),
    Type(Type),
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
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let var = match &self.info {
            HelpInfo::Expression(ty) => Doc::nil()
                .append(theme.expr_var(&self.var.name()))
                .append(Doc::space())
                .append(theme.operator(&":"))
                .append(Doc::softline())
                .append(ty.pretty(theme).group()),
            HelpInfo::Type(ty) => {
                let doc_start = Doc::nil()
                    .append(theme.ty_var(&self.var.name()))
                    .append(Doc::space());
                let doc_builtin = theme.comment(&"(builtin)");

                match ty {
                    Type::Builtin(_) => doc_start.append(doc_builtin),
                    Type::Alias(alias) if matches!(alias.ty(), Type::Builtin(_)) => {
                        doc_start.append(doc_builtin)
                    }
                    Type::Alias(alias) => doc_start
                        .append(theme.operator(&":="))
                        .append(Doc::softline())
                        .append(alias.ty().pretty(theme).group()),
                }
            }
        };

        let documentation = match &self.doc {
            Some(doc) => Doc::hardline().append(doc.pretty(theme)).nest(2),
            None => Doc::nil(),
        };

        var.append(documentation)
    }
}

impl Located for Help {
    fn loc(&self) -> Location {
        self.var.loc()
    }
}
