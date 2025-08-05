use super::super::ty::{Type, Typed, TypedMut};
use super::super::{documentation::WithDoc, Identifier};
use super::Expression;
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Expression Definition
// ==========================================================================
pub struct ExpressionDefinition {
    name: Identifier,
    body: Expression,
}

pub type Definition = WithDoc<ExpressionDefinition>;

impl ExpressionDefinition {
    /// Create a new expression definition
    pub fn new(name: Identifier, body: Expression) -> Self {
        Self { name, body }
    }

    /// Get the name of the expression definition
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    /// Get the body of the expression definition
    pub fn body(&self) -> &Expression {
        &self.body
    }
}

impl Located for ExpressionDefinition {
    /// location is at name of definition
    fn loc(&self) -> Location {
        self.name.loc().union(self.body.loc())
    }
}

impl LocatedSet for ExpressionDefinition {
    fn set_loc(&mut self, loc: &impl Located) {
        self.name.set_loc(loc);
    }
}

pub mod sealed_mut_ty {
    use super::*;
    impl TypedMut for ExpressionDefinition {
        fn ty_mut(&mut self) -> &mut Type {
            self.body.ty_mut()
        }
    }
}
impl Typed for ExpressionDefinition {
    fn ty(&self) -> &Type {
        self.body.ty()
    }
}

impl Pretty for ExpressionDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(theme.keyword(&"Definition"))
            .append(Doc::space())
            .append(theme.def_var(&self.name))
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
