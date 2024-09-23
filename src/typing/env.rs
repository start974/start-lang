use super::ast::*;
use crate::error::*;
use crate::utils::FResult;
use colored::Colorize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypingEnv {
    bindings: HashMap<Ident, Ty>,
    type_alias: HashMap<Ident, Option<Ty>>,
}

const ERROR_TYPE_NOT_FOUND: i32 = 301;

impl TypingEnv {
    /// empty typing environment
    pub fn empty() -> Self {
        Self {
            bindings: HashMap::new(),
            type_alias: HashMap::new(),
        }
    }

    // add binding to typing environment
    pub fn add_binding(mut self, ident: Ident, ty: Ty) -> Self {
        let _ = self.bindings.insert(ident, ty);
        self
    }

    /// normalize type
    pub fn normalize(&self, ty: &Ty) -> Result<Ty, ErrorBox> {
        match &ty.kind {
            Kind::Var(x) => match self.type_alias.get(x) {
                Some(Some(ty)) => self.normalize(ty),
                Some(None) => Ok(ty.clone()),
                None => {
                    let msg = Head::new()
                        .text("Type name")
                        .quoted(&x.to_string())
                        .text("not found");
                    let err = Error::make(msg, ERROR_TYPE_NOT_FOUND).copy_location(ty);
                    Err(Box::new(err))
                }
            },
        }
    }
    // add types in type set
    pub fn add_alias(self, name: Ident, alias_ty: Option<Ty>) -> FResult<Self, (), ErrorBox> {
        let opt_ty_res = match &alias_ty {
            Some(ty) => self.normalize(ty).map(Some),
            None => Ok(None),
        };
        FResult::make(self, opt_ty_res).and_then(|mut env, opt_ty| {
            let _ = env.type_alias.insert(name, opt_ty);
            FResult::ok(env, ())
        })
    }

    // get type of binding
    pub fn get_binding(&self, ident: &Ident) -> Option<Ty> {
        self.bindings.get(ident).cloned()
    }
}

impl std::fmt::Display for TypingEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", "Bindings :".bold())?;
        for (ident, ty) in &self.bindings {
            writeln!(
                f,
                "{}\t:\t{}",
                ident.to_string().blue(),
                ty.to_string_colored()
            )?;
        }
        writeln!(f, "{}", "Type alias:".bold())?;
        for (ident, ty) in &self.type_alias {
            match ty {
                Some(ty) => writeln!(
                    f,
                    "{}\t=>\t{}",
                    ident.to_string().blue(),
                    ty.to_string_colored()
                )?,
                None => writeln!(f, "{}\t=>\t{}", ident.to_string().blue(), "⊥".red())?,
            }
        }
        Ok(())
    }
}
