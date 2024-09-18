use super::expression::Expression;
use super::ident::Ident;
use super::ty::Ty;
pub use crate::location::{Located, Location};

#[derive(Debug, Clone)]
pub struct TyDef {
    pub name: Ident,
    pub ty: Ty,
    pub location: Option<Location>,
}

impl TyDef {
    // get name of definition
    pub fn get_name(&self) -> &Ident {
        &self.name
    }

    // get type of definition
    pub fn get_ty(&self) -> &Ty {
        &self.ty
    }
}

impl Located for TyDef {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        self.location = opt_location;
        self
    }
}
