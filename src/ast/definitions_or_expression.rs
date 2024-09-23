use super::expression::Expression;
pub use super::pretty_print::*;
use super::program::Program;

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
