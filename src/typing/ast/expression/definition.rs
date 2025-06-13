use super::super::identifier::Identifier;
use super::super::ty::{Type, Typed, TypedMut};
use super::Expression;
use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Expression Definition
// ==========================================================================
pub struct Definition {
    name: Identifier,
    body: Expression,
}

impl Definition {
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

impl Located for Definition {
    /// location is at name of definition
    fn loc(&self) -> &Location {
        self.name.loc()
    }
}

impl LocatedSet for Definition {
    fn set_loc(&mut self, loc: &impl Located) {
        self.name.set_loc(loc);
    }
}

pub mod sealed_mut_ty {
    use super::*;
    impl TypedMut for Definition {
        fn ty_mut(&mut self) -> &mut Type {
            self.body.ty_mut()
        }
    }
}
impl Typed for Definition {
    fn ty(&self) -> &Type {
        self.body.ty()
    }
}

impl Pretty for Definition {
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
