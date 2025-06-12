use std::rc::Rc;

use super::super::error::ErrorVariableNotFound;
use super::identifier::Identifier;
use super::ty::{Ty, Typed, TypedMut};
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::{ExpressionDefinition, TyEnv};
// ==========================================================================
// Variable
// ==========================================================================

pub struct Variable {
    /// identifier of the variable
    identifier: Identifier,
    /// type of the variable
    ty: Ty,
}

impl Variable {
    /// get the identifier of the variable
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

pub mod sealed_mut_ty {
    use super::*;
    impl TypedMut for Variable {
        fn ty_mut(&mut self) -> &mut Ty {
            &mut self.ty
        }
    }
}

impl Typed for Variable {
    fn ty(&self) -> &Ty {
        &self.ty
    }
}

impl Located for Variable {
    fn loc(&self) -> &Location {
        &self.identifier.loc()
    }
}

impl LocatedSet for Variable {
    fn set_loc(&mut self, loc: &impl Located) {
        self.identifier.set_loc(loc);
    }
}

impl Pretty for Variable {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.expr_var(&self.identifier.to_string())
    }
}

// ==========================================================================
// Variable Builder
// ==========================================================================
pub struct VariableEnv(TyEnv);

impl VariableEnv {
    /// create a new type environment
    pub fn new() -> Self {
        Self(TyEnv::new())
    }

    /// insert a type into the environment
    pub fn add(&mut self, identifier: Identifier, ty: Ty) {
        self.0.add(identifier, ty);
    }

    /// Get type of identifier
    pub fn get(&self, identifier: &Identifier) -> Result<Variable, ErrorVariableNotFound> {
        let ty = self.0.get(identifier)?;
        Ok(Variable {
            identifier: identifier.clone(),
            ty: ty.clone(),
        })
    }
}
