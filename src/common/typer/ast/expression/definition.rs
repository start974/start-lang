use super::super::ty::{Type, Typed};
use super::super::Pattern;
use super::Expression;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Expression Definition
// ==========================================================================
pub struct Definition {
    /// name of definition
    pattern: Pattern,

    /// body of definition
    body: Expression,
}

impl Definition {
    /// Create a new expression definition
    pub fn new(pattern: Pattern, body: Expression) -> Self {
        Self { pattern, body }
    }

    /// get pattern of definition
    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Get the body of the expression definition
    pub fn body(&self) -> &Expression {
        &self.body
    }
}

/*pub mod sealed_mut_ty {*/
/*use super::*;*/
/*impl TypedMut for Definition {*/
/*fn ty_mut(&mut self) -> &mut Type {*/
/*self.body.ty_mut()*/
/*}*/
/*}*/
/*}*/
impl Typed for Definition {
    fn ty(&self) -> &Type {
        self.body.ty()
    }
}

impl Pretty for Definition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(theme.keyword(&"Definition"))
            .append(Doc::space())
            .append(self.pattern.pretty(theme))
            .append(Doc::group(
                Doc::nil().append(
                    Doc::line()
                        .append(theme.operator(&":"))
                        .append(Doc::space())
                        .append(self.ty().pretty(theme))
                        .group()
                        .nest(4),
                ),
            ))
            .append(Doc::space())
            .append(theme.operator(&":="))
            .append(Doc::line().append(self.body.pretty(theme).group()).nest(2))
    }
}

impl Located for Definition {
    fn loc(&self) -> Location {
        self.pattern.loc().union(self.body.loc())
    }
}
