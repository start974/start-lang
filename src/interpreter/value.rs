use crate::ast::{Ident, NConst, Ty};
use crate::location::Location;
use crate::utils::colored::*;

#[derive(Debug, Clone)]
pub enum Value {
    N(NConst),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::N(n) => write!(f, "{n}"),
        }
    }
}

impl Colored for Value {
    fn colored(&self) -> String {
        match self {
            Self::N(n) => cformat!("<green>{n}</>"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DefValue {
    pub name: Ident,
    pub ty: Ty,
    pub value: Value,
    pub location: Option<Location>,
}

impl std::fmt::Display for DefValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{name} : {ty} := {value}",
            name = self.name,
            ty = self.ty,
            value = self.value
        )
    }
}
impl Colored for DefValue {
    fn colored(&self) -> String {
        let name = &self.name;
        let ty = &self.ty;
        let value = &self.value.colored();
        cformat!("<blue><bold>{name}</></> <red>:</> <yellow>{ty}</> <red>:=</> {value}")
    }
}
