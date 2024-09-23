use super::constant::Constant;
use super::ident::Ident;
pub use super::pretty_print::*;
use crate::location::{Located, Location};

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

impl<TyT> Pretty for Expression<TyT> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::group(match &self.kind {
            ExpressionKind::Const(c) => c.pretty(theme),
            ExpressionKind::Var(x) => theme.expr_var(x),
        })
    }
}
