use crate::ty::{Type, Typed, TypedMut};
use location::{Span, Spanned, SpannedSet};
use pp::prelude::*;

mod constant;
mod definition;

pub use crate::env::Variable as ExpressionVariable;
pub use constant::{Constant, ConstantKind, NConst};
pub use definition::Definition as ExpressionDefinition;

// ==========================================================================
// Expression
// ==========================================================================

pub enum Expression {
    Constant(Constant),
    Variable(ExpressionVariable),
}

mod sealed_mut_ty {
    use super::*;
    impl TypedMut for Expression {
        fn ty_mut(&mut self) -> &mut Type {
            match self {
                Expression::Constant(c) => c.ty_mut(),
                Expression::Variable(v) => v.ty_mut(),
            }
        }
    }
}

impl Typed for Expression {
    fn ty(&self) -> &Type {
        match self {
            Expression::Constant(c) => c.ty(),
            Expression::Variable(v) => v.ty(),
        }
    }
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::Constant(c) => c.span(),
            Expression::Variable(v) => v.span(),
        }
    }
}

impl SpannedSet for Expression {
    fn set_span(&mut self, span: Span) {
        match self {
            Expression::Constant(c) => c.set_span(span),
            Expression::Variable(v) => v.set_span(span),
        }
    }
}

impl Pretty for Expression {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Expression::Constant(c) => c.pretty(theme),
            Expression::Variable(v) => v.pretty(theme),
        }
    }
}
