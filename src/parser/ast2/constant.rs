use crate::location2::{Loc, Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

pub type NConst = BigUint;

pub enum ConstantKind {
    N(NConst),
}

pub struct Constant<Path>(Loc<Path, ConstantKind>);

impl<Path> Constant<Path> {
    pub fn make_n(v: NConst, loc: Location<Path>) -> Self {
        let data = ConstantKind::N(v);
        Self(Loc::new(data, loc))
    }

    pub fn kind(&self) -> &ConstantKind {
        &self.0.data
    }
}

impl<Path> Located<Path> for Constant<Path> {
    fn loc(&self) -> &Location<Path> {
        &self.0.loc
    }
}

impl<Path> Pretty for Constant<Path> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self.kind() {
            ConstantKind::N(n) => theme.number(n.to_string()),
        }
    }
}
