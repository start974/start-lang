use super::super::{Identifier, Ty};
use super::Expression;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================
// Expression Definition
// ==========================================================

pub struct ExpressionDefinition {
    name: Identifier,
    body: Expression,
    ty: Option<Ty>,
}

impl ExpressionDefinition {
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
    pub fn ty(&self) -> &Option<Ty> {
        &self.ty
    }

    /// Set type
    pub fn set_ty(&mut self, ty: Ty) {
        self.ty = Some(ty);
    }
}

impl Located for ExpressionDefinition {
    /// location is at name of definition
    fn loc(&self) -> &Location {
        self.name.loc()
    }
}

impl Pretty for ExpressionDefinition {
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
