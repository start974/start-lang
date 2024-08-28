use super::super::location::{Located, Location};

use super::ident::Ident;
use std::fmt;
/*use super::expression::Expr;*/
/*use super::location::OptLoc;*/
/*use super::Ty::Ty;*/

/// expression definition
pub struct ExprDef {
    pub name: Ident,
    //expr: Expr,
    //ty: Ty,
    location: Option<Location>,
}

impl ExprDef {
    pub fn new(name: &Ident, location: &Option<Location>) -> Self {
        ExprDef {
            name: name.clone(),
            location: location.clone(),
        }
    }
}

impl Located for ExprDef {
    fn location(&self) -> &Option<Location> {
        &self.location
    }
}

impl fmt::Display for ExprDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "def {} : := ", self.name)
    }
}

/// definition
pub enum Definition {
    ExprDef(ExprDef),
}

impl Definition {
    /// make expression definition
    pub fn new_expr_def(ident: &Ident, location: &Option<Location>) -> Self {
        Definition::ExprDef(ExprDef::new(ident, location))
    }

    pub fn get_name(&self) -> &Ident {
        match self {
            Definition::ExprDef(expr_def) => &expr_def.name,
        }
    }
}

impl Located for Definition {
    fn location(&self) -> &Option<Location> {
        match self {
            Definition::ExprDef(expr_def) => expr_def.location(),
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::ExprDef(expr_def) => write!(f, "{}", expr_def),
        }
    }
}
