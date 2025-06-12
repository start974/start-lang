use std::rc::Rc;

use super::super::ty::{Ty, TyBuiltin, Typed, TypedMut};
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
    ty: Ty,
    /// location of the constant
    loc: Location,
    /// kind of the constant
    kind: ConstantKind,
}

impl Constant {
    /// create natural number constant
    pub fn n(v: NConst) -> Self {
        Self {
            ty: Ty::Builtin(TyBuiltin::N),
            loc: UNKNOWN_LOCATION,
            kind: ConstantKind::N(v),
        }
    }

    /// create boolean constant
    pub fn b(b: bool) -> Self {
        Self {
            ty: Ty::Builtin(TyBuiltin::B),
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
            ConstantKind::N(n) => theme.number(n),
            ConstantKind::B(b) => {
                if *b {
                    theme.keyword(&"true")
                } else {
                    theme.keyword(&"false")
                }
            }
        }
    }
}

pub mod sealed_ty_mut {
    use super::*;
    impl TypedMut for Constant {
        fn ty_mut(&mut self) -> &mut Ty {
            &mut self.ty
        }
    }
}

impl Typed for Constant {
    fn ty(&self) -> &Ty {
        &self.ty
    }
}
