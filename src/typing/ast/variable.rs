use super::super::error::ErrorVariableNotFound;
use super::identifier::Identifier;
use super::ty::{Ty, Typed, TypedMut};
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use std::rc::Rc;

use super::ExpressionDefinition;
// ==========================================================================
// Variable
// ==========================================================================

pub struct Variable {
    /// identifier of the variable
    identifier: Identifier,
    /// type of the variable
    ty: Ty,
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
pub trait VariableEnv {
    /// Add definition to builder
    fn add_definition(&mut self, def: ExpressionDefinition);

    /// with definition
    fn with_definition(mut self, def: ExpressionDefinition) -> Self
    where
        Self: Sized,
    {
        self.add_definition(def);
        self
    }

    /// get type of identifier
    fn get_ty(&self, identifier: &Identifier) -> Option<&Ty>;

    /// make a new variable with identifier and type
    fn get_var(&self, identifier: &Identifier) -> Result<Variable, ErrorVariableNotFound> {
        self.get_ty(&identifier)
            .map(|ty| Variable {
                identifier: identifier.clone(),
                ty: ty.clone(),
            })
            .ok_or_else(|| ErrorVariableNotFound::new(identifier.clone()))
    }
}
