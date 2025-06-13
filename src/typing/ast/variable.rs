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

pub mod sealed_mut_ty {
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
pub struct VariableEnv(TypeEnv);

impl VariableEnv {
    /// create a new type environment
    pub fn new() -> Self {
        Self(TypeEnv::new())
    }

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

impl Default for VariableEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Pretty for VariableEnv {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(Doc::text("{"))
            .append(Doc::space())
            .append(Doc::group(Doc::intersperse(
                self.0.iter().map(|(name, ty)| {
                    Doc::group(
                        Doc::nil()
                            .append(theme.expr_var(name))
                            .append(Doc::space())
                            .append(theme.op_typed_by())
                            .append(Doc::space())
                            .append(ty.pretty(theme)),
                    )
                }),
                Doc::text(",").append(Doc::space()),
            )))
            .append(Doc::text("}"))
    }
}
