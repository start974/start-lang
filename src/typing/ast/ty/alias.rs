use crate::utils::location::{Located, LocatedSet, Location, UNKNOWN_LOCATION};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::super::Identifier;
use super::ty::Ty;
use std::collections::HashMap;
use std::rc::Rc;

// ==========================================================================
// alias Ty
// ==========================================================================
#[derive(Debug, Clone)]
pub struct Alias {
    /// name of alias
    name: Identifier,
    /// type of alias
    ty: Rc<Ty>,
    /// location of alias
    loc: Location,
}

impl Alias {
    /// get type of alias
    pub fn ty(&self) -> &Ty {
        self.ty.as_ref()
    }

    /// get rc type
    pub fn rc_ty(&self) -> Rc<Ty> {
        self.ty.clone()
    }

    /// name of alias
    pub fn name(&self) -> &Identifier {
        &self.name
    }
}

impl Located for Alias {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

impl LocatedSet for Alias {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc().clone();
    }
}

impl Pretty for Alias {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.ty_var(&self.name)
    }
}

// ==========================================================================
// Type alias environment
// ==========================================================================
pub struct EnvTy{
    env: HashMap<Identifier, Rc<Ty>>,
}

impl EnvTy {
    /// make a new type alias environment
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    /// insert a type alias into the environment
    pub fn insert(&mut self, name: Identifier, ty: Rc<Ty>) {
        self.env.insert(name, ty);
    }

    /// get a type alias by name
    pub fn get(&self, name: &Identifier) -> Option<&Rc<Ty>> {
        self.env.get(name)
    }

    /// make alias type
    pub fn alias_ty(&self, name: &Identifier) -> Option<Alias> {
        self.get(name).map(|ty| Alias {
            name: name.clone(),
            ty: ty.clone(),
            loc: UNKNOWN_LOCATION,
        })
    }
}
