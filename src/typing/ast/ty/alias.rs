use super::super::Identifier;
use super::{Type, TypeEnv, Typed};
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

impl Located for Alias {
    fn loc(&self) -> &Location {
        self.name.loc()
    }
}

impl LocatedSet for Alias {
    fn set_loc(&mut self, loc: &impl Located) {
        self.name.set_loc(loc);
    }
}

impl Pretty for Alias {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.ty_var(&self.name)
    }
}

// ==========================================================================
// Type alias environment
// ==========================================================================
pub struct TypeAliasEnv(TypeEnv);

impl TypeAliasEnv {
    /// create type alias environment
    pub fn new() -> Self {
        TypeAliasEnv(TypeEnv::new())
    }

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

impl Default for TypeAliasEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Pretty for TypeAliasEnv {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(Doc::text("{"))
            .append(Doc::space())
            .append(Doc::group(Doc::intersperse(
                self.0.iter().map(|(name, ty)| {
                    Doc::group(
                        Doc::nil()
                            .append(theme.ty_var(name))
                            .append(Doc::space())
                            .append(theme.op_eq_def())
                            .append(Doc::space())
                            .append(ty.pretty(theme)),
                    )
                }),
                Doc::line_(),
            )))
            .append(Doc::text("}"))
    }
}
