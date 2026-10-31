use crate::location::{Span, Spanned, SpannedSet};
use crate::tir::error::Error;
pub use alias::Alias;
pub use builtin::Builtin;
use pp::pretty::*;

pub mod alias;
pub mod builtin;

#[derive(Debug, Clone)]
pub enum Type {
    Builtin(Builtin),
    Alias(Alias),
}

impl Type {
    /// type is compatible with another type
    pub fn is_compatible(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl Pretty for Type {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Type::Builtin(builtin) => builtin.pretty(theme),
            Type::Alias(alias) => alias.pretty(theme),
        }
    }
}

impl Spanned for Type {
    fn span(&self) -> Span {
        match self {
            Type::Builtin(builtin) => builtin.span(),
            Type::Alias(alias) => alias.span(),
        }
    }
}

impl SpannedSet for Type {
    fn set_span(&mut self, span: Span) {
        match self {
            Type::Builtin(builtin) => builtin.set_span(span),
            Type::Alias(alias) => alias.set_span(span),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Builtin(b1), Type::Builtin(b2)) => b1 == b2,
            (Type::Alias(a), ty) | (ty, Type::Alias(a)) => a.ty() == ty,
        }
    }
}

impl Eq for Type {}

// ==========================================================================
// Typed Trait
// ==========================================================================
pub trait TypedMut {
    /// get the type
    fn ty_mut(&mut self) -> &mut Type;
}

pub trait Typed {
    /// get the type
    fn ty(&self) -> &Type;

    /// restrict object type to other type
    fn restrict_ty(mut self, ty: Type) -> Result<Self, Error>
    where
        Self: Sized + TypedMut,
    {
        if self.ty().is_compatible(&ty) {
            *self.ty_mut() = ty;
            Ok(self)
        } else {
            Err(Error::UnexpectedType {
                found: Box::new(self.ty().clone()),
                span: ty.span(),
                expected: Box::new(ty),
            })
        }
    }
}
