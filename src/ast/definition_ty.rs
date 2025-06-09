use super::{Ident, Pretty, Ty};
pub use crate::location::{Located, Location};
use crate::utils::theme::{Doc, Theme};

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

    //// get name of definition
    //pub fn get_name(&self) -> &Ident {
    //&self.name
    //}

    //// get type of definition
    //pub fn get_ty(&self) -> &Ty {
    //&self.ty
    //}
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

impl Pretty for TyDef {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::group(
            Doc::nil()
                .append(theme.kw_type())
                .append(Doc::space())
                .append(theme.def_var(&self.name))
                .append(Doc::space())
                .append(theme.op_eq_def())
                .append(Doc::line())
                .append(Doc::group(self.ty.pretty(theme))),
        )
    }
}
