use crate::location2::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::{Constant, Identifier, Ty};

pub enum ExpressionKind<Path> {
    Const(Constant<Path>),
    Var(Identifier<Path>),
}

pub struct Expression<Path> {
    /// kind of expression
    kind: ExpressionKind<Path>,
    /// type of restiction
    ty: Option<Ty<Path>>,
}

impl<Path> Expression<Path> {
    /// Create a new expression with a constant
    pub fn constant(c: Constant<Path>) -> Self {
        Self {
            kind: ExpressionKind::Const(c),
            ty: None,
        }
    }

    /// Create a new expression with a variable
    pub fn var(x: Identifier<Path>) -> Self {
        Self {
            kind: ExpressionKind::Var(x),
            ty: None,
        }
    }

    /// get the kind of the expression
    pub fn kind(&self) -> &ExpressionKind<Path> {
        &self.kind
    }

    /// Get the kind of the expression
    pub fn ty(&self) -> &Option<Ty<Path>> {
        &self.ty
    }

    /// set the type of the expression
    pub fn set_ty(&mut self, ty: Ty<Path>) {
        self.ty = Some(ty);
    }

    /// set the type of the expression and return self
    pub fn with_ty(mut self, ty: Ty<Path>) -> Self {
        self.set_ty(ty);
        self
    }
}

impl<Path> Located<Path> for Expression<Path> {
    fn loc(&self) -> &Location<Path> {
        match &self.kind {
            ExpressionKind::Const(c) => c.loc(),
            ExpressionKind::Var(x) => x.loc(),
        }
    }
}

impl<Path> Pretty for Expression<Path> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let doc = match &self.kind {
            ExpressionKind::Const(c) => c.pretty(theme),
            ExpressionKind::Var(x) => theme.expr_var(x.name()),
        };
        if let Some(ty) = &self.ty {
            doc.append(Doc::space())
                .append(theme.op_typed_by())
                .append(Doc::space())
                .append(ty.pretty(theme))
        } else {
            doc
        }
    }
}
