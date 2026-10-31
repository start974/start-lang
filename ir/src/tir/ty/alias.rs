use crate::location::{Span, Spanned, SpannedSet};
use crate::tir::{Identifier, Type, Typed};
use pp::pretty::*;

// ==========================================================================
// Type Alias
// ==========================================================================
/// use of a type variable, with the type it stands for
#[derive(Debug, Clone)]
pub struct Alias {
    /// name of alias
    name: Identifier,
    /// type of alias
    ty: Box<Type>,
    /// span of alias
    span: Span,
}

impl Alias {
    pub(crate) fn new(name: Identifier, ty: Type, span: Span) -> Self {
        Self {
            name,
            ty: Box::new(ty),
            span,
        }
    }
}

impl Typed for Alias {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl Pretty for Alias {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.ty_var(&self.name.name())
    }
}

impl Spanned for Alias {
    fn span(&self) -> Span {
        self.span
    }
}

impl SpannedSet for Alias {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
