pub use super::super::location::{Located, Location};
use super::expression::Expression;
use super::ident::Ident;

/// definition
pub enum Definition<TyT> {
    ExprDef {
        name: Ident,
        body: Expression<TyT>,
        ty: TyT,
        location: Option<Location>,
    },
}


impl<TyT> Located for Definition<TyT> {
    fn get_location(&self) -> &Option<Location> {
        match self {
            Self::ExprDef { location, .. } => location,
        }
    }

    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        match &mut self {
            Self::ExprDef { location: loc, .. } => *loc = opt_location,
        }
        self
    }
}

