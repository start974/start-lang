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
        Doc::nil()
            .append(theme.def_var(&self.name))
            .append(
                (match &self.ty {
                    Some(ty) => Doc::softline()
                        .append(theme.op_typed_by())
                        .append(Doc::space())
                        .append(ty.pretty(theme).group())
                        .group()
                        .nest(4),
                    None => Doc::nil(),
                })
                .group(),
            )
            .append(Doc::space())
            .append(theme.op_eq_def())
            .append(Doc::softline().append(self.body.pretty(theme).group()).nest(2))
    }
}
