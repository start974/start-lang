use crate::tir::{Identifier, Type, Typed};
use location::{Span, Spanned, SpannedSet};
use pp::pretty::*;

// ==========================================================================
// Expression Variable
// ==========================================================================

/// use of an expression variable, with the type of its definition
pub struct Variable {
    /// identifier of the variable
    identifier: Identifier,
    /// type of the variable
    ty: Type,
    /// span of the variable
    span: Span,
}

impl Variable {
    pub(crate) fn new(identifier: Identifier, ty: Type, span: Span) -> Self {
        Self {
            identifier,
            ty,
            span,
        }
    }

    /// get the identifier of the variable
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }
}

mod sealed_mut_ty {
    use crate::tir::TypedMut;

    use super::*;
    impl TypedMut for Variable {
        fn ty_mut(&mut self) -> &mut Type {
            &mut self.ty
        }
    }
}

impl Typed for Variable {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl Spanned for Variable {
    fn span(&self) -> Span {
        self.span
    }
}

impl SpannedSet for Variable {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl Pretty for Variable {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.expr_var(&self.identifier)
    }
}
