use super::super::Type;
use super::{Expression, Pattern};
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================
// Expression Definition
// ==========================================================

#[derive(Debug)]
pub struct Definition {
    pattern: Pattern,
    body: Expression,
    ty: Option<Type>,
}

impl Definition {
    /// Create a new expression definition
    pub fn new(pattern: Pattern, body: Expression) -> Self {
        Self {
            pattern,
            body,
            ty: None,
        }
    }

    /// Get pattern
    pub fn pattern(&self) -> &Pattern {
        &self.pattern
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
        self.pattern.loc()
    }
}

impl Pretty for Definition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(self.pattern.pretty(theme))
            .append(
                (match &self.ty {
                    Some(ty) => Doc::softline()
                        .append(theme.operator(&":"))
                        .append(Doc::space())
                        .append(ty.pretty(theme).group())
                        .group()
                        .nest(4),
                    None => Doc::nil(),
                })
                .group(),
            )
            .append(Doc::space())
            .append(theme.operator(&":="))
            .append(
                Doc::softline()
                    .append(self.body.pretty(theme).group())
                    .nest(2),
            )
    }
}
