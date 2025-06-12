use super::super::super::error::ErrorUnexpectedType;
use super::alias::Alias;
use super::builtin::Builtin;
use crate::typing::ast::Identifier;
use crate::utils::location::Located;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use std::collections::HashMap;
use std::rc::Rc;

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
    pub fn aliases(&self) -> HashMap<Identifier, Rc<Ty>> {
        fn aux(ty: &Ty, mut acc: HashMap<Identifier, Rc<Ty>>) -> HashMap<Identifier, Rc<Ty>> {
            match ty {
                Ty::Alias(alias) => {
                    acc.insert(alias.name().clone(), alias.rc_ty());
                    aux(alias.ty(), acc)
                }
                Ty::Builtin(_) => acc,
            }
        }
        aux(self, HashMap::new())
    }

    /// remove alias from type
    pub fn normalize(&self) -> Self {
        match self {
            Ty::Alias(alias) => alias.ty().normalize(),
            Ty::Builtin(builtin) => Self::Builtin(builtin.clone()),
        }
    }

    /// type is compatible with another type
    pub fn is_compatible(&self, other: &Self) -> bool {
        *self == *other
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
struct TypeEnv {
    /// map of identifiers to types
    env: HashMap<Identifier, Ty>,
}

impl TypeEnv {
    /// create a new type environment
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }
    /// insert a type into the environment
    pub fn insert(&mut self, name: Identifier, ty: Ty) {
        self.env.insert(name, ty);
    }

    /// get a type from the environment
    pub fn get(&self, name: &Identifier) -> Option<&Ty> {
        self.env.get(name)
    }
}
