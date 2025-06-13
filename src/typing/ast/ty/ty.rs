use super::super::super::error::{ErrorUnexpectedType, ErrorVariableNotFound};
use super::alias::Alias;
use super::builtin::Builtin;
use crate::typing::ast::Identifier;
use crate::utils::location::Located;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use std::collections::HashMap;

// ==========================================================================
// alias Ty
// ==========================================================================

#[derive(Debug, Clone)]
pub enum Ty {
    Builtin(Builtin),
    Alias(Alias),
}

impl Ty {
    //// get map of aliases
    /*    pub fn aliases(&self) -> HashMap<Identifier, &'_ Ty> {*/
    /*fn aux<'a>(*/
    /*ty: &'a Ty,*/
    /*mut acc: HashMap<Identifier, &'a Ty>,*/
    /*) -> HashMap<Identifier, &'a Ty> {*/
    /*match ty {*/
    /*Ty::Alias(alias) => {*/
    /*acc.insert(alias.name().clone(), alias.ty());*/
    /*aux(alias.ty(), acc)*/
    /*}*/
    /*Ty::Builtin(_) => acc,*/
    /*}*/
    /*}*/
    /*aux(self, HashMap::new())*/
    /*}*/

    ///// remove alias from type
    //pub fn normalize(&self) -> Self {
    //match self {
    //Ty::Alias(alias) => alias.ty().normalize(),
    //Ty::Builtin(builtin) => Self::Builtin(builtin.clone()),
    //}
    //}

    /// type is compatible with another type
    pub fn is_compatible(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl From<Builtin> for Ty {
    fn from(builtin: Builtin) -> Self {
        Ty::Builtin(builtin)
    }
}

impl From<Alias> for Ty {
    fn from(alias: Alias) -> Self {
        Ty::Alias(alias)
    }
}

impl Pretty for Ty {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Ty::Builtin(builtin) => builtin.pretty(theme),
            Ty::Alias(alias) => alias.pretty(theme),
        }
    }
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Ty::Builtin(b1), Ty::Builtin(b2)) => b1 == b2,
            (Ty::Alias(a), ty) | (ty, Ty::Alias(a)) => a.ty() == ty,
        }
    }
}

impl Eq for Ty {}

// ==========================================================================
// Typed Trait
// ==========================================================================
pub trait TypedMut {
    /// get the type
    fn ty_mut(&mut self) -> &mut Ty;
}

pub trait Typed {
    /// get the type
    fn ty(&self) -> &Ty;

    /// constrait type to other type
    /// permit to use alias
    fn constraint_ty(mut self, ty: Ty) -> Result<Self, ErrorUnexpectedType>
    where
        Self: Located + Sized + TypedMut,
    {
        if self.ty().is_compatible(&ty) {
            *self.ty_mut() = ty;
            Ok(self)
        } else {
            Err(ErrorUnexpectedType::new(self.ty(), &ty, self.loc()))
        }
    }
}

// ==========================================================================
// Type Environment
// ==========================================================================
pub struct TyEnv(HashMap<Identifier, Ty>);

impl TyEnv {
    /// create a new type environment
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// insert a type into the environment
    pub fn add(&mut self, name: Identifier, ty: Ty) {
        if self.0.insert(name.clone(), ty).is_some() {
            panic!("Identifier {name:#?} already exists in environment")
        }
    }

    /// Get type of identifier
    pub fn get(&self, identifier: &Identifier) -> Result<&Ty, ErrorVariableNotFound> {
        self.0
            .get(identifier)
            .ok_or_else(|| ErrorVariableNotFound::new(identifier.clone()))
    }

    /// iter on the environment
    pub fn iter(&self) -> impl Iterator<Item = (&Identifier, &Ty)> {
        self.0.iter()
    }
}

impl Default for TyEnv {
    fn default() -> Self {
        Self::new()
    }
}
