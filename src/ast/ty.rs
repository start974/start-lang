use super::ident::Ident;
pub use super::pretty_print::*;
pub use crate::location::{Located, Location};
use std::hash::{Hash, Hasher};

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

impl Pretty for Ty {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self.kind {
            Kind::Var(x) => theme.ty_var(x),
        }
    }
}
impl Eq for Ty {}

impl Hash for Ty {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}
