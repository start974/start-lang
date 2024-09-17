pub use crate::ast::*;
use crate::stdlib;
use crate::utils::colored::*;
use std::fmt;

pub trait Typed {
    /// get type
    fn get_ty(&self) -> &Ty;
}

pub type TExpression = Expression<Ty>;
pub type TDefinition = ExprDef<Ty>;
pub type TProgram = Program<TDefinition>;
pub type TDefsOrExpr = DefsOrExpr<TDefinition, Ty>;

/* ------------------------------------------------------------------------ */
/* Constant */
/* ------------------------------------------------------------------------ */
impl Typed for Constant {
    fn get_ty(&self) -> &Ty {
        match self {
            Constant::N(_) => &stdlib::number_n::N_TYPE,
        }
    }
}

/* ------------------------------------------------------------------------ */
/* Expression */
/* ------------------------------------------------------------------------ */

impl TExpression {
    /// make constant expression
    pub fn make_constant(c: Constant) -> Self {
        Self {
            ty: c.get_ty().clone(),
            kind: ExpressionKind::Const(c),
            location: None,
        }
    }

    /// make variable expression
    pub fn make_var(x: Ident, ty: Ty) -> Self {
        Self {
            ty,
            kind: ExpressionKind::Var(x),
            location: None,
        }
    }
}

impl Typed for TExpression {
    fn get_ty(&self) -> &Ty {
        &self.ty
    }
}

/* ------------------------------------------------------------------------ */
/* Definition */
/* ------------------------------------------------------------------------ */

impl TDefinition {
    /// make expression definition
    pub fn make_expr_def(name: Ident, body: TExpression) -> Self {
        Self {
            name,
            ty: body.get_ty().clone(),
            body,
            location: None,
        }
    }
}

impl Typed for TDefinition {
    fn get_ty(&self) -> &Ty {
        &self.ty
    }
}

impl fmt::Display for TDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let body = &self.body;
        let ty = &self.ty;
        write!(f, "def {name} : {ty} := {body}")
    }
}

impl Colored for TDefinition {
    fn colored(&self) -> String {
        let name = cformat!("<blue>{}</>", self.name);
        let body = self.body.colored();
        let ty = self.ty.colored();
        cformat!("<magenta>def</magenta> {name} <red>:</> {ty} <red>:=</> {body}")
    }
}
