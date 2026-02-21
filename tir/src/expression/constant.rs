use super::super::ty::{Type, TypeBuiltin, Typed, TypedMut};
use location::{GetSpan, SetSpan, Span};
use num_bigint::BigUint;
use pp::pretty::*;

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
    /// span of the constant
    span: Span,
}

impl Constant {
    fn new(kind: ConstantKind, ty: Type) -> Self {
        Self {
            kind,
            ty,
            span: Span::default(),
        }
    }
    /// create natural number constant
    pub fn nat(v: NConst) -> Self {
        Self::new(ConstantKind::Nat(v), Type::Builtin(TypeBuiltin::nat()))
    }

    /// create boolean constant
    pub fn boolean(b: bool) -> Self {
        Self::new(ConstantKind::Bool(b), Type::Builtin(TypeBuiltin::bool()))
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

impl GetSpan for Constant {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Constant {
    fn set_span(&mut self, span: Span) {
        self.span = span;
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
