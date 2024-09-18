use super::expression::Expression;
use super::ident::Ident;
use super::ty::Ty;
pub use crate::location::{Located, Location};
use crate::utils::colored::*;

#[derive(Debug, Clone)]
pub struct TyDef {
    pub name: Ident,
    pub ty: Ty,
    pub location: Option<Location>,
}

impl TyDef {
    /// make a new definition
    pub fn new(name: Ident, ty: Ty) -> Self {
        Self {
            name,
            ty,
            location: None,
        }
    }

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

impl std::fmt::Display for TyDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "type {} := {}", self.name, self.ty)
    }
}

impl Colored for TyDef {
    fn colored(&self) -> String {
        cformat!(
            "<magenta>type</magenta> <yellow>{}</> <red>:=</> {}",
            self.name,
            self.ty.colored()
        )
    }
}
