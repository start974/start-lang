use super::super::error::{ErrorUnexpectedType, ErrorVariableNotFound};
use crate::typing::ast::Identifier;
use crate::utils::location::Located;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use std::collections::HashMap;

mod alias;
mod builtin;

pub use alias::Alias as TypeAlias;
pub use alias::TypeAliasEnv;
pub use builtin::Builtin as TypeBuiltin;

#[derive(Debug, Clone)]
pub enum Type {
    Builtin(TypeBuiltin),
    Alias(TypeAlias),
}

impl Type {
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

impl From<TypeBuiltin> for Type {
    fn from(builtin: TypeBuiltin) -> Self {
        Type::Builtin(builtin)
    }
}

impl From<TypeAlias> for Type {
    fn from(alias: TypeAlias) -> Self {
        Type::Alias(alias)
    }
}

impl Pretty for Type {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Type::Builtin(builtin) => builtin.pretty(theme),
            Type::Alias(alias) => alias.pretty(theme),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Builtin(b1), Type::Builtin(b2)) => b1 == b2,
            (Type::Alias(a), ty) | (ty, Type::Alias(a)) => a.ty() == ty,
        }
    }
}

impl Eq for Type {}

// ==========================================================================
// Typed Trait
// ==========================================================================
pub trait TypedMut {
    /// get the type
    fn ty_mut(&mut self) -> &mut Type;
}

pub trait Typed {
    /// get the type
    fn ty(&self) -> &Type;

    /// restrict object type to other type
    fn restrict_ty(mut self, ty: Type) -> Result<Self, ErrorUnexpectedType>
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
#[derive(Debug)]
pub struct TypeEnv(HashMap<Identifier, Type>);

impl TypeEnv {
    /// create a new type environment
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// insert a type into the environment
    pub fn add(&mut self, name: Identifier, ty: Type) {
        if self.0.insert(name.clone(), ty).is_some() {
            panic!("Identifier {name:#?} already exists in environment")
        }
    }

    /// Get type of identifier
    pub fn get(&self, identifier: &Identifier) -> Result<&Type, ErrorVariableNotFound> {
        self.0
            .get(identifier)
            .ok_or_else(|| ErrorVariableNotFound::new(identifier.clone()))
    }

    /// iter on the environment
    pub fn iter(&self) -> impl Iterator<Item = (&Identifier, &Type)> {
        self.0.iter()
    }
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self::new()
    }
}
