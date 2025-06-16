use super::Expression;
use crate::{
    parser::ast::Type,
    utils::{
        location::{Located, Location},
        pretty::Pretty,
        theme::{Doc, Theme},
    },
};

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
    pub fn new(ty: Type, expr: Expression) -> Self {
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
            .append(theme.op_typed_by())
            .append(self.ty.pretty(theme))
    }
}
