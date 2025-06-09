use super::ast::*;
use crate::error::*;
use crate::utils::theme::{Doc, Theme};
use crate::utils::FResult;
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

impl Pretty for TypingEnv {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(theme.title1("Bindings :"))
            .append(Doc::line())
            .append(Doc::intersperse(
                self.bindings.iter().map(|(ident, ty)| {
                    Doc::group(
                        Doc::nil()
                            .append(Doc::text(ident.to_string()))
                            .append(Doc::space())
                            .append(Doc::text(":"))
                            .append(Doc::space())
                            .append(ty.pretty(theme)),
                    )
                }),
                Doc::line(),
            ))
            .append(Doc::line())
            .append(theme.title1("Type alias:"))
            .append(Doc::line())
            .append(Doc::intersperse(
                self.type_alias.iter().map(|(ident, ty)| match ty {
                    Some(ty) => Doc::group(
                        Doc::nil()
                            .append(Doc::text(ident.to_string()))
                            .append(Doc::space())
                            .append(Doc::text("=>"))
                            .append(Doc::space())
                            .append(ty.pretty(theme)),
                    ),
                    None => Doc::group(
                        Doc::nil()
                            .append(Doc::text(ident.to_string()))
                            .append(Doc::space())
                            .append(Doc::text("=>"))
                            .append(Doc::space())
                            .append(Doc::text("⊥")),
                    ),
                }),
                Doc::line(),
            ))
    }
}
