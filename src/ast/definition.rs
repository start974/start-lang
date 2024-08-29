use super::super::location::{Located, Location};

use super::ident::Ident;
use std::fmt;
/*use super::expression::Expr;*/
/*use super::location::OptLoc;*/
/*use super::Ty::Ty;*/

/// definition
pub enum Definition {
    ExprDef {
        name: Ident,
        //expr: Expr,
        //ty: Ty,
        location: Option<Location>,
    },
}

impl Definition {
    /// make expression definition
    pub fn make_expr_def(name: Ident, location: Option<Location>) -> Self {
        Self::ExprDef { name, location }
    }

    /// get identifier name of definition
    pub fn get_name(&self) -> &Ident {
        match self {
            Definition::ExprDef { name, .. } => name,
        }
    }
}

impl Located for Definition {
    fn location(&self) -> &Option<Location> {
        match self {
            Definition::ExprDef { location, .. } => location,
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::ExprDef { name, .. } => write!(f, "def {name} : := "),
        }
    }
}
