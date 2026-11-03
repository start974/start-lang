use crate::tir::Type;
use crate::tir::ty::BuiltinType;
use num_bigint::BigUint;
use pp::pretty::*;

// ============================================================================
// Constant
// ============================================================================

/// constant, the builtin ones evaluated (`__Constant_true__` is `Bool(true)`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    Nat(BigUint),
    Bool(bool),
    Char(char),
}

impl Constant {
    /// type of the constant
    pub fn ty(&self) -> Type {
        Type::Builtin(match self {
            Constant::Nat(_) => BuiltinType::Nat,
            Constant::Bool(_) => BuiltinType::Bool,
            Constant::Char(_) => BuiltinType::Char,
        })
    }
}

impl Pretty for Constant {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Constant::Nat(n) => theme.number(n),
            Constant::Bool(b) => theme.boolean(*b),
            Constant::Char(c) => theme.character(*c),
        }
    }
}
