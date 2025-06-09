use super::{expression::Expression, Pretty};
use super::program::Program;
use crate::utils::theme::{Doc, Theme};

pub enum DefsOrExpr<TDef, TyT> {
    Definitions(Program<TDef>),
    Expression(Expression<TyT>),
}

impl<TDef, TyT> Pretty for DefsOrExpr<TDef, TyT>
where
    TDef: Pretty,
    Expression<TyT>: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            DefsOrExpr::Definitions(prog) => prog.pretty(theme),
            DefsOrExpr::Expression(expr) => expr.pretty(theme),
        }
    }
}
