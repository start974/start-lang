use crate::typer::ast;
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
            Self::Nat(n) => theme.number(n),
            Self::Bool(b) => theme.boolean(*b),
            Self::Char(c) => theme.character(*c),
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
