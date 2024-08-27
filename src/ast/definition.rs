use super::ident::Ident;
use std::fmt;
/*use super::expression::Expr;*/
/*use super::location::OptLoc;*/
/*use super::Ty::Ty;*/

/// expression definition
pub struct ExprDef {
    pub ident: Ident,
    //expr: Expr,
    //ty: Ty,
    //loc: OptLoc,
}

impl ExprDef {
    pub fn make(ident: Ident) -> Self {
        ExprDef { ident }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.ident)
    }
}

impl fmt::Display for ExprDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// definition
pub enum Definition {
    ExprDef(ExprDef),
}

impl Definition {
    /// make expression definition
    pub fn make_expr_def(ident: Ident) -> Self {
        let expr_def = ExprDef::make(ident);
        Definition::ExprDef(expr_def)
    }

    pub fn to_string(&self) -> String {
        match self {
            Definition::ExprDef(expr_def) => expr_def.to_string(),
        }
    }

    pub fn get_name(&self) -> &String {
        match self {
            Definition::ExprDef(expr_def) => &expr_def.ident.name,
        }
    }

    pub fn get_ident(&self) -> &Ident {
        match self {
            Definition::ExprDef(expr_def) => &expr_def.ident,
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}
