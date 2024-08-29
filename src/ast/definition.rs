use super::super::location::{Located, Location};

use super::expression::Expression;
use super::ident::Ident;
use super::ty::{Ty, Typed};
use std::fmt;
/*use super::location::OptLoc;*/
/*use super::Ty::Ty;*/

/// definition
pub enum Definition<TyT> {
    ExprDef {
        name: Ident,
        body: Expression<TyT>,
        ty: TyT,
        location: Option<Location>,
    },
}

impl<TyT> Definition<TyT> {
    /// get identifier name of definition
    pub fn get_name(&self) -> &Ident {
        match self {
            Definition::ExprDef { name, .. } => name,
        }
    }
    /*
    pub fn get_body(&self) -> &Expression<TyT> {
        match self {
            Definition::ExprDef { body, .. } => body,
        }
    }
    */
}

impl<TyT> Located for Definition<TyT> {
    fn get_location(&self) -> &Option<Location> {
        match self {
            Self::ExprDef { location, .. } => location,
        }
    }

    fn set_location(mut self, location: Location) -> Self {
        match &mut self {
            Self::ExprDef { location: loc, .. } => *loc = Some(location),
        }
        self
    }
}

impl Definition<Option<Ty>> {
    /// make expression definition
    pub fn make_expr_def(name: Ident, body: Expression<Option<Ty>>) -> Self {
        Self::ExprDef {
            name,
            ty: None,
            body,
            location: None,
        }
    }
    // set type with option type
    pub fn set_opt_ty(mut self, ty: Option<Ty>) -> Self {
        match &mut self {
            Self::ExprDef { ty: t, .. } => *t = ty,
        }
        self
    }

    /*
    /// set type
    pub fn set_ty(mut self, ty: Ty) -> Self {
        self.set_opt_ty(Some(ty))
    }

    /// get type
    pub fn get_ty(&self) -> &Option<Ty> {
        match self {
            Definition::ExprDef { ty, .. } => ty,
        }
    }
    */
}
impl fmt::Display for Definition<Option<Ty>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::ExprDef { name, body, ty, .. } => match ty {
                None => write!(f, "def {name} := {body}"),
                Some(ty) => write!(f, "def {name} : {ty} := {body}"),
            },
        }
    }
}

/*
impl Definition<Ty> {
    /// make expression definition
    pub fn make_expr_def(name: Ident, ty: Ty, body: Expression<Ty>) -> Self {
        Self::ExprDef {
            name,
            ty,
            body,
            location: None,
        }
    }
}

impl Typed for Definition<Ty> {
    fn get_ty(&self) -> &Ty {
        match self {
            Definition::ExprDef { ty, .. } => ty,
        }
    }
}

impl fmt::Display for Definition<Ty> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::ExprDef { name, body, ty, .. } => write!(f, "def {name} : {ty} := {body}"),
        }
    }
}
*/
