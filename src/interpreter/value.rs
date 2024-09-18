use crate::ast::{Ident, NConst, Ty};
use crate::error::Error;
use crate::location::*;
use crate::utils::colored::*;

#[derive(Debug, Clone)]
pub enum Value {
    N(NConst),
}

const ERROR_CONVERT_TO_INTEGER: i32 = -2;
impl TryInto<i32> for Value {
    type Error = Error;
    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            Self::N(n) => NConst::try_into(n).map_err(|_| {
                Error::make(
                    "value is to loong to be convert to integer",
                    ERROR_CONVERT_TO_INTEGER,
                )
            }),
        }
    }
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
        let name = &self.name;
        let ty = &self.ty;
        let value = &self.value;
        write!(f, "{name} : {ty} := {value}")
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

impl Located for DefValue {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        self.location = opt_location;
        self
    }
}

pub type DefValues = Vec<DefValue>;

pub enum DefsOrValue {
    Value(Value),
    Defs(DefValues),
}

impl std::fmt::Display for DefsOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{}", value),
            Self::Defs(defs) => defs.iter().try_for_each(|def| def.fmt(f)),
        }
    }
}

impl Colored for DefsOrValue {
    fn colored(&self) -> String {
        match self {
            Self::Value(value) => value.colored(),
            Self::Defs(defs) => {
                let mut s = String::new();
                for def in defs {
                    s += &def.colored();
                    s += "\n";
                }
                s
            }
        }
    }
}
