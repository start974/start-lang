use super::super::{Identifier, Type};
use super::Expression;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================
// Expression Definition
// ==========================================================

#[derive(Debug)]
pub struct Definition {
    name: Identifier,
    body: Expression,
    ty: Option<Type>,
}

impl Definition {
    /// Create a new expression definition
    pub fn new(name: Identifier, body: Expression) -> Self {
        Self {
            name,
            body,
            ty: None,
        }
    }

    /// Get the name of the expression definition
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    /// Get the body of the expression definition
    pub fn body(&self) -> &Expression {
        &self.body
    }

    /// Get the type of the expression definition
    pub fn ty(&self) -> &Option<Type> {
        &self.ty
    }

    /// Set type
    pub fn set_ty(&mut self, ty: Type) {
        self.ty = Some(ty);
    }

    /// with type
    pub fn with_ty(mut self, ty: Type) -> Self {
        self.set_ty(ty);
        self
    }
}

impl Located for Definition {
    /// location is at name of definition
    fn loc(&self) -> &Location {
        self.name.loc()
    }
}

impl Pretty for Definition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let doc_ty = match &self.ty {
            Some(ty) => Doc::nil()
                .append(Doc::line())
                .append(theme.op_typed_by())
                .append(Doc::space())
                .append(ty.pretty(theme)),
            None => Doc::nil(),
        };
        Doc::nil()
            .append(theme.kw_def()) // NOTE: rm when command definition implemented
            .append(Doc::space())
            .append(theme.def_var(&self.name)) // NOTE: change if using in let
            .append(Doc::group(doc_ty))
            .append(theme.op_eq_def())
            .append(Doc::line())
            .append(Doc::group(self.body.pretty(theme)))
    }
}
