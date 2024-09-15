use super::constant::Constant;
pub use crate::location::{Located, Location};

/// constant expression
#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Const(Constant),
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
