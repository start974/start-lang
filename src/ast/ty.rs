pub use super::super::location::{Located, Location};
use super::ident::Ident;

/// constant types
#[derive(Debug, Clone)]
pub enum Kind {
    Var(Ident),
}

#[derive(Debug, Clone)]
pub struct Ty {
    kind: Kind,
    location: Option<Location>,
}

impl Ty {
    /// make variable type
    pub fn make_var(ident: Ident) -> Self {
        Self {
            kind: Kind::Var(ident),
            location: None,
        }
    }
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            Kind::Var(ident) => write!(f, "{}", ident),
        }
    }
}


impl Located for Ty {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        self.location = opt_location;
        self
    }
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (Kind::Var(ident1), Kind::Var(ident2)) => ident1 == ident2,
        }
    }
}

impl Eq for Ty {}
