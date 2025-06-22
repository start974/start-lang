use super::super::error::ErrorVariableNotFound;
use super::identifier::Identifier;
use super::ty::{Type, Typed, TypedMut};
use super::TypeEnv;
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Variable
// ==========================================================================

pub struct Variable {
    /// identifier of the variable
    identifier: Identifier,
    /// type of the variable
    ty: Type,
}

impl Variable {
    /// get the identifier of the variable
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

mod sealed_mut_ty {
    use super::*;
    impl TypedMut for Variable {
        fn ty_mut(&mut self) -> &mut Type {
            &mut self.ty
        }
    }
}

impl Typed for Variable {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl Located for Variable {
    fn loc(&self) -> &Location {
        self.identifier.loc()
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
#[derive(Debug, Default)]
pub struct VariableEnv(TypeEnv);

impl VariableEnv {
    /// insert a type into the environment
    pub fn add(&mut self, identifier: Identifier, ty: Type) {
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
