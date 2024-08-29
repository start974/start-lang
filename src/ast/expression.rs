use super::super::location::{Located, Location};
use super::constant;

type NConst = u32;

pub enum Constant {
    N(constant::Constant<NConst>),
}

impl Constant {
    pub fn make_n(v: NConst) -> Self {
        let n = constant::Constant::make(v);
        Self::N(n)
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::N(c) => write!(f, "{c}"),
        }
    }
}

impl Located for Constant {
    fn get_location(&self) -> &Option<Location> {
        match self {
            Self::N(c) => c.get_location(),
        }
    }

    fn set_location(mut self, location: Location) -> Self {
        match self {
            Self::N(mut c) => {
                c = c.set_location(location);
                self = Self::N(c);
            }
        }
        self
    }
}

/// constant expression
pub enum Kind {
    Const(Constant),
}

pub struct Expression {
    kind: Kind,
    location: Option<Location>,
}

impl Expression {
    pub fn make_constant(c: Constant) -> Self {
        Expression {
            kind: Kind::Const(c),
            location: None,
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            Kind::Const(c) => write!(f, "{c}"),
        }
    }
}

impl Located for Expression {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }
    fn set_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }
}
