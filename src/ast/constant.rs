use num_bigint::BigUint;
use crate::utils::colored::*;

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

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::N(c) => write!(f, "{c}"),
        }
    }
}

impl Colored for Constant {
    fn colored(&self) -> String {
        match self {
            Self::N(c) => cformat!("<green>{c}</>"),
        }
    }
}
