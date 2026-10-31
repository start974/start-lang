use ir::lir;
use pp::pretty::*;

// =========================================================================
// Value
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Nat(lir::Nat),
    Bool(bool),
    Char(char),
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

impl From<&lir::Constant> for Value {
    fn from(c: &lir::Constant) -> Self {
        match c {
            lir::Constant::Nat(n) => Self::Nat(n.clone()),
            lir::Constant::Bool(b) => Self::Bool(*b),
            lir::Constant::Char(c) => Self::Char(*c),
        }
    }
}
