pub use super::super::ast::*;
use std::fmt;

type OptionTy = Option<Ty>;

pub trait WeakTyped {
    /// get type
    fn get_opt_ty(&self) -> &OptionTy;

    /// set type
    fn set_opt_ty(self, ty: OptionTy) -> Self;
}

pub type WTExpression = Expression<OptionTy>;
pub type WTDefinition = Definition<OptionTy>;
pub type WTProgram = Program<OptionTy>;

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
}

impl WeakTyped for WTExpression {
    fn set_opt_ty(mut self, ty: OptionTy) -> Self {
        self.ty = ty;
        self
    }

    fn get_opt_ty(&self) -> &OptionTy {
        &self.ty
    }
}

impl std::fmt::Display for WTExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ExpressionKind::Const(c) => write!(f, "{c}"),
        }
    }
}

/* ------------------------------------------------------------------------ */
/* Definition */
/* ------------------------------------------------------------------------ */

impl WTDefinition {
    /// make expression definition
    pub fn make_expr_def(name: Ident, body: WTExpression) -> Self {
        Self::ExprDef {
            name,
            ty: None,
            body,
            location: None,
        }
    }
    // set type with option type
}

impl WeakTyped for WTDefinition {
    fn set_opt_ty(mut self, ty: Option<Ty>) -> Self {
        match &mut self {
            Self::ExprDef { ty: t, .. } => *t = ty,
        }
        self
    }

    fn get_opt_ty(&self) -> &OptionTy {
        match self {
            Definition::ExprDef { ty, .. } => ty,
        }
    }
}

impl fmt::Display for WTDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::ExprDef { name, body, ty, .. } => match ty {
                None => write!(f, "def {name} := {body}"),
                Some(ty) => write!(f, "def {name} : {ty} := {body}"),
            },
        }
    }
}
