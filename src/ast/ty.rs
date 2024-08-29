use super::super::location::{Located, Location};
use super::ident::Ident;

/// constant types
pub enum Kind {
    Var(Ident),
}

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

    fn set_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }
}

pub trait Typed {
    /// get type
    fn get_ty(&self) -> &Ty;
}

pub trait WeakTyped {
    /// get type
    fn get_opt_ty(&self) -> &Option<Ty>;

    /// set type
    fn set_ty(self, ty: Ty) -> Self;
}
