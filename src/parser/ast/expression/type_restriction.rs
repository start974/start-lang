use super::Expression;
use crate::{
    parser::ast::Type,
    utils::{
        location::{Located, Location},
        pretty::Pretty,
        theme::{Doc, Theme},
    },
};

#[derive(Debug)]
pub struct TypeRestriction {
    /// The expression that must conform to the type
    expr: Box<Expression>,
    /// The type to which the restriction applies
    ty: Type,
    /// location of type restriction
    loc: Location,
}

impl TypeRestriction {
    /// Creates a new type restriction with the given type and expression.
    pub fn new(expr: Expression, ty: Type) -> Self {
        let loc = ty.loc().union(expr.loc());
        Self {
            ty,
            expr: Box::new(expr),
            loc,
        }
    }

    /// get type of restriction
    pub fn ty(&self) -> &Type {
        &self.ty
    }

    /// get expression
    pub fn expression(&self) -> &Expression {
        &self.expr
    }
}

impl Located for TypeRestriction {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

impl Pretty for TypeRestriction {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(self.expr.pretty(theme))
            .append(Doc::space())
            .append(theme.op_typed_by())
            .append(Doc::space())
            .append(self.ty.pretty(theme))
    }
}
