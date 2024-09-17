use super::expression::Expression;
use super::program::Program;
use crate::utils::colored::*;

pub enum DefsOrExpr<TDef, TyT> {
    Definitions(Program<TDef>),
    Expression(Expression<TyT>),
}

impl<TDef, TyT> std::fmt::Display for DefsOrExpr<TDef, TyT>
where
    TDef: std::fmt::Display,
    Expression<TyT>: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DefsOrExpr::Definitions(prog) => write!(f, "{}", prog),
            DefsOrExpr::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

impl<TDef, TyT> Colored for DefsOrExpr<TDef, TyT>
where
    TDef: Colored,
    Expression<TyT>: Colored,
{
    fn colored(&self) -> String {
        match self {
            DefsOrExpr::Definitions(prog) => prog.colored(),
            DefsOrExpr::Expression(expr) => expr.colored(),
        }
    }
}
