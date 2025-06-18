use super::super::ty::{Type, TypeBuiltin, Typed, TypedMut};
use crate::utils::format_number;
use crate::utils::location::{Located, LocatedSet, Location, UNKNOWN_LOCATION};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

pub type NConst = BigUint;

pub enum ConstantKind {
    N(NConst),
    B(bool),
}

pub struct Constant {
    /// type of constant
    ty: Type,
    /// location of the constant
    loc: Location,
    /// kind of the constant
    kind: ConstantKind,
}

impl Constant {
    /// create natural number constant
    pub fn n(v: NConst) -> Self {
        Self {
            ty: Type::from(TypeBuiltin::N),
            loc: UNKNOWN_LOCATION,
            kind: ConstantKind::N(v),
        }
    }

    /// create boolean constant
    pub fn b(b: bool) -> Self {
        Self {
            ty: Type::Builtin(TypeBuiltin::B),
            loc: UNKNOWN_LOCATION,
            kind: ConstantKind::B(b),
        }
    }

    /// get kind of the constant
    pub fn kind(&self) -> &ConstantKind {
        &self.kind
    }
}

impl Located for Constant {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

impl LocatedSet for Constant {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc().clone();
    }
}

impl Pretty for Constant {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self.kind() {
            ConstantKind::N(n) => theme.constant(&format_number(n)),
            ConstantKind::B(b) => theme.constant(b),
        }
    }
}

pub mod sealed_ty_mut {
    use super::*;
    impl TypedMut for Constant {
        fn ty_mut(&mut self) -> &mut Type {
            &mut self.ty
        }
    }
}

impl Typed for Constant {
    fn ty(&self) -> &Type {
        &self.ty
    }
}
