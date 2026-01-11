use pp::prelude::*;
use tir::{Constant, NConst};

// =========================================================================
// Value
// =========================================================================

#[derive(Debug, Clone)]
pub enum Value {
    Nat(NConst),
    Bool(bool),
    Char(char),
}

impl Value {
    pub fn nat(n: NConst) -> Self {
        Value::Nat(n)
    }

    pub fn bool(b: bool) -> Self {
        Value::Bool(b)
    }

    pub fn char(c: char) -> Self {
        Value::Char(c)
    }
}

impl Pretty for Value {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Self::Nat(n) => theme.number(n),
            Self::Bool(b) => theme.boolean(*b),
            Self::Char(c) => theme.character(*c),
        }
    }
}

impl From<&Constant> for Value {
    fn from(c: &Constant) -> Self {
        use tir::ConstantKind;
        match c.kind() {
            ConstantKind::Nat(n) => Self::nat(n.clone()),
            ConstantKind::Bool(b) => Self::bool(*b),
            ConstantKind::Char(c) => Self::char(*c),
        }
    }
}
