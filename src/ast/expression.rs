use super::constant::Constant;
use super::ident::Ident;
use crate::location::{Located, Location};
use crate::utils::colored::*;

/// constant expression
#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Const(Constant),
    Var(Ident),
}

#[derive(Debug, Clone)]
pub struct Expression<TyT> {
    pub kind: ExpressionKind,
    pub ty: TyT,
    pub location: Option<Location>,
}

impl<TyT> Located for Expression<TyT> {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        self.location = opt_location;
        self
    }
}

impl<TyT> std::fmt::Display for Expression<TyT> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ExpressionKind::Const(c) => write!(f, "{c}"),
            ExpressionKind::Var(x) => write!(f, "{x}"),
        }
    }
}

impl<TyT> Colored for Expression<TyT> {
    fn colored(&self) -> String {
        match &self.kind {
            ExpressionKind::Const(c) => c.colored(),
            ExpressionKind::Var(x) => cformat!("<blue>{x}<blue>"),
        }
    }
}
