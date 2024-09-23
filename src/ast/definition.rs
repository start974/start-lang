use super::definition_expression::ExprDef;
use super::definition_ty::TyDef;
pub use crate::location::{Located, Location};

/// definition
#[derive(Debug, Clone)]
pub enum Definition<TyT> {
    ExprDef(ExprDef<TyT>),
    TyDef(TyDef),
}

impl<TyT> Located for Definition<TyT> {
    fn get_location(&self) -> &Option<Location> {
        match self {
            Self::ExprDef(expr_def) => expr_def.get_location(),
            Self::TyDef(ty_def) => ty_def.get_location(),
        }
    }

    fn set_opt_location(self, opt_location: Option<Location>) -> Self {
        match self {
            Self::ExprDef(expr_def) => Self::ExprDef(expr_def.set_opt_location(opt_location)),
            Self::TyDef(ty_def) => Self::TyDef(ty_def.set_opt_location(opt_location)),
        }
    }
}
