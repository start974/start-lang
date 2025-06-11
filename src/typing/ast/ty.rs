use super::super::error::ErrorUnexpectedType;
use crate::utils::location::{Located, LocatedSet, Location, UNKNOWN_LOCATION};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Ty
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinTy {
    /// natural number type
    N,
    /// relative number type
    Z,
    /// boolean type
    B,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind {
    Builtin(BuiltinTy),
}

#[derive(Debug, Clone)]
pub struct Ty {
    /// type kind
    kind: TyKind,
    /// location of the type
    loc: Location,
}

impl Ty {
    /// make N type
    pub fn n() -> Self {
        Self {
            kind: TyKind::Builtin(BuiltinTy::N),
            loc: UNKNOWN_LOCATION,
        }
    }

    /// make Z type
    pub fn z() -> Self {
        Self {
            kind: TyKind::Builtin(BuiltinTy::Z),
            loc: UNKNOWN_LOCATION,
        }
    }

    /// make boolean type
    pub fn bool() -> Self {
        Self {
            kind: TyKind::Builtin(BuiltinTy::B),
            loc: UNKNOWN_LOCATION,
        }
    }

    pub fn loc_mut(&mut self) -> &mut Location {
        &mut self.loc
    }

    /// type kind
    pub fn kind(&self) -> &TyKind {
        &self.kind
    }
}

impl Located for Ty {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

impl LocatedSet for Ty {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc().clone();
    }
}

impl Pretty for Ty {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self.kind {
            TyKind::Builtin(BuiltinTy::N) => theme.ty_var(&"ℕ"),
            TyKind::Builtin(BuiltinTy::Z) => theme.ty_var(&"ℤ"),
            TyKind::Builtin(BuiltinTy::B) => theme.ty_var(&"𝔹"),
        }
    }
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for Ty {}

// ==========================================================================
// Typed Trait
// ==========================================================================

pub trait Typed {
    /// get the type
    fn ty(&self) -> &Ty;

    /// get mutate type location
    fn ty_loc_mut(&mut self) -> &mut Location;

    /// constraint the type
    fn constraint_ty(mut self, ty: &Ty) -> Result<Self, ErrorUnexpectedType>
    where
        Self: Sized + Located,
    {
        if ty == self.ty() {
            *self.ty_loc_mut() = ty.loc().clone();
            Ok(self)
        } else {
            Err(ErrorUnexpectedType::new(
                ty,
                self.ty(),
                self.loc(),
            ))
        }
    }
}
