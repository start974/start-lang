use crate::utils::location::{Located, LocatedSet, Location, UNKNOWN_LOCATION};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ==========================================================================
// Builtin Ty
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// natural number type
    N,
    ///// relative number type
    //Z,
    ///// boolean type
    //B,
}

#[derive(Debug, Clone)]
pub struct Builtin {
    /// kind of type
    kind: Kind,

    /// location of builtin type
    loc: Location,
}

impl Builtin {
    /// make n type
    pub fn n() -> Self {
        Self {
            kind: Kind::N,
            loc: UNKNOWN_LOCATION,
        }
    }
}

impl Located for Builtin {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

impl LocatedSet for Builtin {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc().clone();
    }
}

impl Pretty for Builtin {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self.kind {
            Kind::N => theme.ty_var(&"ℕ"),
            //Kind::Z => theme.ty_var(&"ℤ"),
            //Kind::B => theme.ty_var(&"𝔹"),
        }
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for Builtin {}
