use super::location::{OptLoc, Located};

/// primitive types
pub enum TyPrim {
    TyN(OptLoc)
}

impl TyPrim {
    /// pretty print
    pub fn pretty(&self) -> String {
        match self {
            TyPrim::TyN(_) => String::from("N"),
        }
    }
}

impl Located for TyPrim {
    pub fn localisation(&self) -> OptLoc {
        match self {
            TyPrim::TyN(loc) => loc.clone(),
        }
    }
}

/// constant types
pub enum Ty {
    Prim(TyPrim)
}

pub impl Ty {

    /// make natural type
    pub fn nat<T>(extra : T, loc: OptLoc) -> Ty {
        Ty::Prim(TyPrim::TyN(loc))
    }

    /// pretty print
    pub fn pretty(&self) -> String {
        match self {
            Ty::Prim(p_ty) => p_ty.pretty(),
        }
    }
}

trait Typed {
    /// get type
    pub fn type(&self) -> Ty;

    /// pretty print without type
    pub fn pretty_without_type(&self) -> String;
}
