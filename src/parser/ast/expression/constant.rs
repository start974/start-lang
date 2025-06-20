use crate::utils::format_number;
use crate::utils::location::{Loc, Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

#[derive(Debug)]
pub enum ConstantKind {
    Nat(BigUint),
    Char(char),
}

#[derive(Debug)]
pub struct Constant(Loc<ConstantKind>);

impl Constant {
    /// make a nat constant
    pub fn nat(v: BigUint, loc: Location) -> Self {
        let data = ConstantKind::Nat(v);
        Self(Loc::new(data, loc))
    }

    /// make a char constant
    pub fn char(c: char, loc: Location) -> Self {
        let data = ConstantKind::Char(c);
        Self(Loc::new(data, loc))
    }

    /// get the kind of the constant
    pub fn kind(&self) -> &ConstantKind {
        &self.0.data
    }
}

impl Located for Constant {
    fn loc(&self) -> &Location {
        &self.0.loc
    }
}

impl Pretty for Constant {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self.kind() {
            ConstantKind::Nat(n) => theme.constant(&format_number(n)),
            ConstantKind::Char(c) => theme.constant(&format!("'{}'", c.escape_default())),
        }
    }
}
