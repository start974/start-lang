use crate::typing::ast;
use crate::utils::format_number;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub enum Constant {
    //B(bool),
    N(BigUint),
}

impl Pretty for Constant {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Self::N(n) => theme.constant(&format_number(n)),
            //Self::B(b) => theme.constant(b),
        }
    }
}

impl From<&ast::Constant> for Constant {
    fn from(c: &ast::Constant) -> Self {
        match c.kind() {
            ast::ConstantKind::N(n) => Constant::N(n.clone()),
            //ast::ConstantKind::B(b) => Constant::B(*b),
        }
    }
}
