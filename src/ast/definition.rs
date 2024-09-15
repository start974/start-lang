use super::expression::Expression;
use super::ident::Ident;
pub use crate::location::{Located, Location};

#[derive(Debug, Clone)]
pub struct ExprDef<TyT> {
    pub name: Ident,
    pub body: Expression<TyT>,
    pub ty: TyT,
    pub location: Option<Location>,
}

impl<TyT> ExprDef<TyT> {
    // get name of definition
    pub fn get_name(&self) -> &Ident {
        &self.name
    }

    // get body of definition
    pub fn get_body(&self) -> &Expression<TyT> {
        &self.body
    }
}

impl<TyT> Located for ExprDef<TyT> {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        self.location = opt_location;
        self
    }
}

/// definition
#[derive(Debug, Clone)]
pub enum Definition<TyT> {
    ExprDef(ExprDef<TyT>),
}

impl<TyT> Located for Definition<TyT> {
    fn get_location(&self) -> &Option<Location> {
        match self {
            Self::ExprDef(expr_def) => expr_def.get_location(),
        }
    }

    fn set_opt_location(self, opt_location: Option<Location>) -> Self {
        match self {
            Self::ExprDef(expr_def) => Self::ExprDef(expr_def.set_opt_location(opt_location)),
        }
    }
}
