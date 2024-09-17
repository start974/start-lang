pub use crate::ast::*;

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
//-----------------------------------------------------------------------------
// Definition
//-----------------------------------------------------------------------------

impl WTDefinition {
    /// make expression definition
    pub fn make_expr_def(name: Ident, body: WTExpression) -> Self {
        let expr_def = ExprDef {
            name,
            ty: None,
            body,
            location: None,
        };
        Self::ExprDef(expr_def)
    }
}

impl WeakTyped for WTDefinition {
    fn set_opt_ty(self, opt_ty: Option<Ty>) -> Self {
        match self {
            Self::ExprDef(expr_def) => Self::ExprDef(expr_def.set_opt_ty(opt_ty)),
        }
    }

    fn get_opt_ty(&self) -> &OptionTy {
        match self {
            Definition::ExprDef(expr_def) => expr_def.get_opt_ty(),
        }
    }
}

impl fmt::Display for WTDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::ExprDef(expr_def) => expr_def.fmt(f),
        }
    }
}
