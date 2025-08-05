use super::super::Identifier;
use super::{Type, TypeBuiltin, TypeEnv, Typed};
use crate::typing::ast::Documentation;
use crate::typing::error::ErrorVariableNotFound;
use crate::utils::location::{Located, LocatedSet, Location};
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

impl Located for Alias {
    fn loc(&self) -> Location {
        self.name.loc()
    }
}

impl LocatedSet for Alias {
    fn set_loc(&mut self, loc: &impl Located) {
        self.name.set_loc(loc);
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

    /// get builtin type
    pub fn builtin_ty(&self, name: &str) -> Option<TypeBuiltin> {
        match name {
            "__Type_Nat__" => Some(TypeBuiltin::nat()),
            "__Type_Bool__" => Some(TypeBuiltin::bool()),
            "__Type_Char__" => Some(TypeBuiltin::char()),
            _ => None,
        }
    }

    /// get alias by name
    pub fn get(&self, name: &Identifier) -> Result<Type, ErrorVariableNotFound> {
        if let Some(builtin_ty) = self.builtin_ty(name.name()) {
            Ok(Type::Builtin(builtin_ty))
        } else {
            let ty = self.0.get(name)?;
            let alias = Alias {
                name: name.clone(),
                ty: Box::new(ty.clone()),
            };
            Ok(Type::Alias(alias))
        }
    }

    /// add documentation
    pub fn add_doc(&mut self, name: Identifier, doc: Documentation) {
        self.0.add_doc(name, doc);
    }

    /// get documentation
    pub fn get_doc(&self, name: &Identifier) -> Option<&Documentation> {
        self.0.doc.get(name)
    }
}
