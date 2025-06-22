use super::super::Identifier;
use super::{Type, TypeEnv, Typed};
use crate::typing::error::ErrorVariableNotFound;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// alias Ty
// ==========================================================================
#[derive(Debug, Clone)]
pub struct Alias {
    /// name of alias
    name: Identifier,
    /// type of alias
    ty: Box<Type>,
}

impl Typed for Alias {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl Pretty for Alias {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.ty_var(&self.name.name())
    }
}

// ==========================================================================
// Type alias environment
// ==========================================================================
#[derive(Debug, Default)]
pub struct TypeAliasEnv(TypeEnv);

impl TypeAliasEnv {
    /// add alias to environment
    pub fn add(&mut self, name: Identifier, ty: Type) {
        self.0.add(name, ty)
    }

    /// get alias by name
    pub fn get(&self, name: &Identifier) -> Result<Alias, ErrorVariableNotFound> {
        let ty = self.0.get(name)?;
        Ok(Alias {
            name: name.clone(),
            ty: Box::new(ty.clone()),
        })
    }
}
