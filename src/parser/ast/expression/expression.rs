use super::super::{Identifier, Ty};
use super::Constant;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub enum ExpressionKind {
    Constant(Constant),
    Variable(Identifier),
}

pub struct Expression {
    /// kind of expression
    kind: ExpressionKind,
    /// type of restiction
    ty: Option<Ty>,
}

impl Expression {
    /// Create a new expression with a constant
    pub fn constant(c: Constant) -> Self {
        Self {
            kind: ExpressionKind::Constant(c),
            ty: None,
        }
    }

    /// Create a new expression with a variable
    pub fn var(x: Identifier) -> Self {
        Self {
            kind: ExpressionKind::Variable(x),
            ty: None,
        }
    }

    /// get the kind of the expression
    pub fn kind(&self) -> &ExpressionKind {
        &self.kind
    }

    /// Get the kind of the expression
    pub fn ty(&self) -> &Option<Ty> {
        &self.ty
    }
}

impl Located for Expression {
    fn loc(&self) -> &Location {
        match &self.kind {
            ExpressionKind::Constant(c) => c.loc(),
            ExpressionKind::Variable(x) => x.loc(),
        }
    }
}

impl Pretty for Expression {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let doc = match &self.kind {
            ExpressionKind::Constant(c) => c.pretty(theme),
            ExpressionKind::Variable(x) => theme.expr_var(&x.to_string()),
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
