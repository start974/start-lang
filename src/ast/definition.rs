use super::super::location::{Located, Location};

use super::expression::Expression;
use super::ident::Ident;
use std::fmt;
/*use super::location::OptLoc;*/
/*use super::Ty::Ty;*/

/// definition
pub enum Definition {
    ExprDef {
        name: Ident,
        body: Expression,
        //ty: Ty,
        location: Option<Location>,
    },
}

impl Definition {
    /// make expression definition
    pub fn make_expr_def(name: Ident, body: Expression) -> Self {
        Self::ExprDef {
            name,
            location: None,
            body,
        }
    }

    /// get identifier name of definition
    pub fn get_name(&self) -> &Ident {
        match self {
            Definition::ExprDef { name, .. } => name,
        }
    }
}

impl Located for Definition {
    fn get_location(&self) -> &Option<Location> {
        match self {
            Definition::ExprDef { location, .. } => location,
        }
    }

    fn set_location(mut self, location: Location) -> Self {
        match &mut self {
            Definition::ExprDef { location: loc, .. } => *loc = Some(location),
        }
        self
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::ExprDef { name, body, .. } => write!(f, "def {name} := {body}"),
        }
    }
}
