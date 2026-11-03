use num_bigint::BigUint;
use pp::pretty::*;

/// natural number, unbounded
pub type Nat = BigUint;

// ============================================================================
// Constant
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    Nat(Nat),
    Bool(bool),
    Char(char),
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
