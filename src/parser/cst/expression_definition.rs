use super::{operator, Expression, Pattern, Type};
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ============================================================================
// Optional Type
// ============================================================================
#[derive(Debug)]
pub struct TypedBy {
    colon: operator::Colon,
    ty: Type,
}

impl TypedBy {
    pub fn new(colon: operator::Colon, ty: Type) -> Self {
        Self { colon, ty }
    }

    /// get type
    pub fn get_type(&self) -> &Type {
        &self.ty
    }
}

impl Pretty for TypedBy {
    fn pretty(&self, theme: &Theme) -> Doc {
        Doc::nil()
            .append(self.colon.pretty(theme))
            .append(Doc::space())
            .append(self.ty.pretty(theme))
    }
}

// ============================================================================
// Expression Definition
// ============================================================================
#[derive(Debug)]
pub struct ExpressionDefinition {
    pattern: Pattern,
    typed_by: Option<TypedBy>,
    eq_def: operator::EqDef,
    expr: Expression,
}

impl ExpressionDefinition {
    pub fn new(
        pattern: Pattern,
        typed_by: Option<TypedBy>,
        eq_def: operator::EqDef,
        expr: Expression,
    ) -> Self {
        Self {
            pattern,
            typed_by,
            eq_def,
            expr,
        }
    }

    /// get pattern
    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// get optal type of definition
    pub fn typed_by(&self) -> Option<&Type> {
        match self.typed_by {
            Some(ref typed_by) => Some(typed_by.get_type()),
            None => None,
        }
    }

    /// get body
    pub fn body(&self) -> &Expression {
        &self.expr
    }
}

impl Located for ExpressionDefinition {
    fn loc(&self) -> Location {
        self.pattern.loc().union(self.expr.loc())
    }
}

impl Pretty for ExpressionDefinition {
    fn pretty(&self, theme: &Theme) -> Doc {
        let doc_typed_by = {
            match &self.typed_by {
                Some(typed_by) => Doc::softline()
                    .append(typed_by.pretty(theme))
                    .group()
                    .nest(4),
                None => Doc::nil(),
            }
        };
        let doc_body = Doc::softline()
            .append(self.expr.pretty(theme).group())
            .nest(2);
        Doc::nil()
            .append(self.pattern.pretty(theme))
            .append(doc_typed_by)
            .append(Doc::space())
            .append(self.eq_def.pretty(theme))
            .append(doc_body)
    }
}
