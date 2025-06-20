use crate::typing::ast;
use crate::utils::format_number;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub enum Constant {
    Nat(BigUint),
    Bool(bool),
    Char(char),
}

impl Pretty for Constant {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Self::Nat(n) => theme.constant(&format_number(n)),
            Self::Bool(b) => theme.constant(b),
            Self::Char(c) => theme.constant(&format!("'{}'", c.escape_default())),
        }
    }
}

impl From<&ast::Constant> for Constant {
    fn from(c: &ast::Constant) -> Self {
        match c.kind() {
            ast::ConstantKind::Nat(n) => Constant::Nat(n.clone()),
            ast::ConstantKind::Bool(b) => Constant::Bool(*b),
            ast::ConstantKind::Char(c) => Constant::Char(*c),
        }
    }
}
