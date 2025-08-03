use super::super::ty::{Type, TypeBuiltin, Typed, TypedMut};
use crate::utils::location::{Located, LocatedSet, Location};
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
    /// kind of the constant
    kind: ConstantKind,
    /// type of constant
    ty: Type,
    /// location of the constant
    loc: Location,
}

impl Constant {
    fn new(kind: ConstantKind, ty: Type) -> Self {
        Self {
            kind,
            ty,
            loc: Location::unknown(),
        }
    }
    /// create natural number constant
    pub fn nat(v: NConst) -> Self {
        Self::new(ConstantKind::Nat(v), Type::from(TypeBuiltin::nat()))
    }

    /// create boolean constant
    pub fn boolean(b: bool) -> Self {
        Self::new(ConstantKind::Bool(b), Type::from(TypeBuiltin::bool()))
    }

    /// create a character constant
    pub fn character(c: char) -> Self {
        Self::new(ConstantKind::Char(c), Type::Builtin(TypeBuiltin::char()))
    }

    /// get kind of the constant
    pub fn kind(&self) -> &ConstantKind {
        &self.kind
    }
}

impl Located for Constant {
    fn loc(&self) -> Location {
        self.loc.clone()
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
            ConstantKind::Nat(n) => theme.number(n),
            ConstantKind::Bool(b) => theme.boolean(*b),
            ConstantKind::Char(c) => theme.character(*c),
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
