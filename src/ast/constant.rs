use super::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

pub type NConst = BigUint;

#[derive(Debug, Clone)]
pub enum Constant {
    N(NConst),
}

impl Constant {
    pub fn make_n(v: NConst) -> Self {
        Self::N(v)
    }
}

impl Pretty for Constant {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self {
            Self::N(n) => theme.number(n),
        }
    }
}
