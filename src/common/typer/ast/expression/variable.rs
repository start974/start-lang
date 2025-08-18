use crate::typer::ast::{Documentation, Identifier, Type, TypeEnv, Typed};
use crate::typer::error::ErrorVariableNotFound;
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::{Constant, Expression};

// ==========================================================================
// Variable
// ==========================================================================

pub struct Variable {
    /// identifier of the variable
    identifier: Identifier,
    /// type of the variable
    ty: Type,
    /// location of varable
    loc: Location,
}

impl Variable {
    /// get the identifier of the variable
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

mod sealed_mut_ty {
    use crate::typer::ast::TypedMut;

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
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl LocatedSet for Variable {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc().clone();
    }
}

impl Pretty for Variable {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.expr_var(&self.identifier)
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

    fn get_builtin_constant(&self, builtin: &str) -> Option<Constant> {
        match builtin {
            "__Constant_true__" => Some(Constant::boolean(true)),
            "__Constant_false__" => Some(Constant::boolean(false)),
            _ => None,
        }
    }

    /// Get expression of identifier (builtin or variable)
    pub fn get(
        &self,
        identifier: &Identifier,
        loc: Location,
    ) -> Result<Expression, ErrorVariableNotFound> {
        match self.get_builtin_constant(identifier.name()) {
            Some(constant) => Ok(Expression::Constant(constant)),
            None => {
                let ty = self.0.get(identifier, loc.clone())?;
                let var = Variable {
                    identifier: identifier.clone(),
                    ty: ty.clone(),
                    loc,
                };
                Ok(Expression::Variable(var))
            }
        }
    }

    /// add documentation for an identifier
    pub fn add_doc(&mut self, identifier: Identifier, doc: Documentation) {
        self.0.add_doc(identifier, doc);
    }

    /// get documentation
    pub fn get_doc(&self, identifier: &Identifier) -> Option<&Documentation> {
        self.0.get_doc(identifier)
    }
}
