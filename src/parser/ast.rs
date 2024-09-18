pub use crate::ast::*;
use crate::utils::colored::*;

use std::fmt;

type OptionTy = Option<Ty>;

pub trait WeakTyped {
    /// get type
    fn get_opt_ty(&self) -> &OptionTy;

    /// set type
    fn set_opt_ty(self, ty: OptionTy) -> Self;
}

pub type WTExpression = Expression<OptionTy>;
pub type WTExprDef = ExprDef<OptionTy>;
pub type WTDefinition = Definition<OptionTy>;
pub type WTProgram = Program<WTDefinition>;
pub type WTDefsOrExpr = DefsOrExpr<WTDefinition, OptionTy>;

/* ------------------------------------------------------------------------ */
/* Expression */
/* ------------------------------------------------------------------------ */

impl WTExpression {
    pub fn make_constant(c: Constant) -> Self {
        Self {
            kind: ExpressionKind::Const(c),
            ty: None,
            location: None,
        }
    }

    pub fn make_var(ident: Ident) -> Self {
        Self {
            kind: ExpressionKind::Var(ident),
            ty: None,
            location: None,
        }
    }
}

impl WeakTyped for WTExpression {
    fn set_opt_ty(mut self, opt_ty: OptionTy) -> Self {
        self.ty = opt_ty;
        self
    }

    fn get_opt_ty(&self) -> &OptionTy {
        &self.ty
    }
}

//-----------------------------------------------------------------------------
// Expression Definition
//-----------------------------------------------------------------------------
impl WTExprDef {
    pub fn new(name: Ident, body: WTExpression) -> Self {
        Self {
            name,
            body,
            ty: None,
            location: None,
        }
    }
}

impl WeakTyped for WTExprDef {
    fn set_opt_ty(mut self, opt_ty: Option<Ty>) -> Self {
        self.ty = opt_ty;
        self
    }

    fn get_opt_ty(&self) -> &OptionTy {
        &self.ty
    }
}

impl fmt::Display for WTExprDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let body = &self.body;
        match &self.ty {
            None => write!(f, "def {name} := {body}"),
            Some(ty) => write!(f, "def {name} : {ty} := {body}"),
        }
    }
}

impl Colored for WTExprDef {
    fn colored(&self) -> String {
        let name = cformat!("<blue>{}</>", self.name);
        let body = self.body.colored();
        match &self.ty {
            None => cformat!("<magenta>def</> {name} <red>:=</> {body}"),
            Some(ty) => {
                let ty = ty.colored();
                cformat!("<magenta>def</> {name} <red>:</> {ty} <red>:=</> {body}")
            }
        }
    }
}
//-----------------------------------------------------------------------------
// Definition
//-----------------------------------------------------------------------------

impl fmt::Display for WTDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExprDef(expr_def) => expr_def.fmt(f),
            Self::TyDef(ty_def) => ty_def.fmt(f),
        }
    }
}

impl Colored for WTDefinition {
    fn colored(&self) -> String {
        match self {
            Self::ExprDef(expr_def) => expr_def.colored(),
            Self::TyDef(ty_def) => ty_def.colored(),
        }
    }
}
