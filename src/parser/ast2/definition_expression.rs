use super::{Identifier, Expression, Ty};
pub use crate::location2::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct ExpressionDefinition<Path> {
    name: Identifier<Path>,
    body: Expression<Path>,
    ty: Option<Ty<Path>>,
}

impl<Path> ExpressionDefinition<Path> {
    /// Create a new expression definition
    pub fn new(name: Identifier<Path>, body: Expression<Path>) -> Self {
        Self {
            name,
            body,
            ty: None,
        }
    }

    /// Get the name of the expression definition
    pub fn name(&self) -> &Identifier<Path> {
        &self.name
    }

    /// Get the body of the expression definition
    pub fn body(&self) -> &Expression<Path> {
        &self.body
    }

    /// Get the type of the expression definition
    pub fn ty(&self) -> &Option<Ty<Path>> {
        &self.ty
    }

    /// Set type
    pub fn set_ty(&mut self, ty: Ty<Path>) {
        self.ty = Some(ty);
    }

    /// with type
    pub fn with_ty(mut self, ty: Ty<Path>) -> Self {
        self.set_ty(ty);
        self
    }
}

impl<Path> Located<Path> for ExpressionDefinition<Path> {
    /// location is at name of definition
    fn loc(&self) -> &Location<Path> {
        self.name.loc()
    }
}

impl<Path> Pretty for ExpressionDefinition<Path> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(theme.kw_def()) // NOTE: rm when command definition implemented
            .append(Doc::space())
            .append(theme.def_var(self.name.name())) // NOTE: change if using in let
            .append(Doc::space())
            .append(match &self.ty {
                None => Doc::nil(),
                Some(ty) => Doc::group(
                    Doc::nil()
                        .append(theme.op_typed_by())
                        .append(Doc::space())
                        .append(ty.pretty(theme))
                        .append(Doc::space()),
                ),
            })
            .append(theme.op_eq_def())
            .append(Doc::space())
            .append(Doc::group(self.body.pretty(theme)))
    }
}
