use super::super::{Identifier, Ty, Typed};
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
    fn loc(&self) -> &Location {
        self.name.loc()
    }
}

impl LocatedSet for ExpressionDefinition {
    fn set_loc(&mut self, loc: &impl Located) {
        self.name.set_loc(loc);
    }
}

impl Typed for ExpressionDefinition {
    fn ty(&self) -> &Ty {
        self.body.ty()
    }

    fn ty_loc_mut(&mut self) -> &mut Location {
        self.body.ty_loc_mut()
    }
}

impl Pretty for ExpressionDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let ty = self.ty();
        let doc_ty = Doc::nil()
            .append(Doc::line())
            .append(theme.op_typed_by())
            .append(Doc::space())
            .append(ty.pretty(theme));
        Doc::nil()
            .append(theme.kw_def()) // NOTE: rm when command definition implemented
            .append(Doc::space())
            .append(theme.def_var(&self.name)) // NOTE: change if using in let
            .append(Doc::group(doc_ty))
            .append(theme.op_eq_def())
            .append(Doc::line())
            .append(Doc::group(self.body.pretty(theme)))
    }
}
