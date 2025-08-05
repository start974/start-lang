use super::{operator, Expression, Pattern, Type};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ============================================================================
// Optional Type
// ============================================================================
#[derive(Debug)]
pub struct TypedBy {
    pub colon: operator::Colon,
    pub ty: Type,
}

impl TypedBy {
    /// get type
    pub fn get_type(&self) -> &Type {
        &self.ty
    }
}

impl Pretty for TypedBy {
    fn pretty(&self, theme: &Theme) -> Doc {
        Doc::nil()
            .append(self.colon.pretty(theme))
            .append(Doc::softline())
            .append(self.ty.pretty(theme))
    }
}

// ============================================================================
// Expression Definition
// ============================================================================
#[derive(Debug)]
pub struct ExpressionDefinition {
    pub pattern: Pattern,
    pub typed_by: Option<TypedBy>,
    pub eq_def: operator::EqDef,
    pub body: Expression,
}

impl ExpressionDefinition {
    /// get optal type of definition
    pub fn typed_by(&self) -> Option<&Type> {
        match self.typed_by {
            Some(ref typed_by) => Some(typed_by.get_type()),
            None => None,
        }
    }
}

/*impl Located for ExpressionDefinition {*/
    /*fn loc(&self) -> Location {*/
        /*self.pattern.loc().union(self.body.loc())*/
    /*}*/
/*}*/

impl Pretty for ExpressionDefinition {
    fn pretty(&self, theme: &Theme) -> Doc {
        let doc_typed_by = {
            match &self.typed_by {
                Some(typed_by) => Doc::softline()
                    .append(typed_by.pretty(theme))
                    .group()
                    .nest(2),
                None => Doc::nil(),
            }
        };

        let doc_body = Doc::softline().append(self.body.pretty(theme).group());

        Doc::nil()
            .append((self.pattern.pretty(theme)).append(doc_typed_by))
            .append(Doc::space())
            .append(self.eq_def.pretty(theme))
            .append(doc_body)
    }
}
