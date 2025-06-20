use super::super::ty::{Type, TypeBuiltin, Typed, TypedMut};
use crate::utils::format_number;
use crate::utils::location::{Located, LocatedSet, Location, UNKNOWN_LOCATION};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

pub type NConst = BigUint;

pub enum ConstantKind {
    Nat(NConst),
    Bool(bool),
    Char(char),
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
    pub fn nat(v: NConst) -> Self {
        Self {
            ty: Type::from(TypeBuiltin::Nat),
            loc: UNKNOWN_LOCATION,
            kind: ConstantKind::Nat(v),
        }
    }

    /// create boolean constant
    pub fn boolean(b: bool) -> Self {
        Self {
            ty: Type::Builtin(TypeBuiltin::Bool),
            loc: UNKNOWN_LOCATION,
            kind: ConstantKind::Bool(b),
        }
    }

    /// create a character constant
    pub fn character(c: char) -> Self {
        Self {
            ty: Type::Builtin(TypeBuiltin::Char),
            loc: UNKNOWN_LOCATION,
            kind: ConstantKind::Char(c),
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
            ConstantKind::Nat(n) => theme.constant(&format_number(n)),
            ConstantKind::Bool(b) => theme.constant(b),
            ConstantKind::Char(c) => theme.constant(&format!("'{}'", c.escape_default())),
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
