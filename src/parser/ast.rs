use super::super::ast;
use std::fmt;

pub type Ident = ast::Ident;
pub type Env = ast::Env;
pub type Ty = ast::Ty;

type OptionTy = Option<Ty>;

pub trait WeakTyped {
    /// get type
    fn get_opt_ty(&self) -> &OptionTy;

    /// set type
    fn set_ty(self, ty: Ty) -> Self;
}

pub type Constant = ast::Constant;
pub type ExpressionKind = ast::ExpressionKind;
pub type Expression = ast::Expression<OptionTy>;
pub type Definition = ast::Definition<OptionTy>;
pub type Program = ast::Program<OptionTy>;

/* ------------------------------------------------------------------------ */
/* Expression */
/* ------------------------------------------------------------------------ */

impl Expression {
    pub fn make_constant(c: Constant) -> Self {
        Self {
            kind: ExpressionKind::Const(c),
            ty: None,
            location: None,
        }
    }
    pub fn set_opt_ty(mut self, ty: OptionTy) -> Self {
        self.ty = ty;
        self
    }
}
impl WeakTyped for Expression {
    fn set_ty(self, ty: Ty) -> Self {
        self.set_opt_ty(Some(ty))
    }

    fn get_opt_ty(&self) -> &OptionTy {
        &self.ty
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ExpressionKind::Const(c) => write!(f, "{c}"),
        }
    }
}

/* ------------------------------------------------------------------------ */
/* Definition */
/* ------------------------------------------------------------------------ */

impl Definition {
    /// make expression definition
    pub fn make_expr_def(name: Ident, body: Expression) -> Self {
        Self::ExprDef {
            name,
            ty: None,
            body,
            location: None,
        }
    }
    // set type with option type
    pub fn set_opt_ty(mut self, ty: Option<Ty>) -> Self {
        match &mut self {
            Self::ExprDef { ty: t, .. } => *t = ty,
        }
        self
    }
}

impl WeakTyped for Definition {
    fn set_ty(self, ty: Ty) -> Self {
        self.set_opt_ty(Some(ty))
    }

    fn get_opt_ty(&self) -> &OptionTy {
        match self {
            Definition::ExprDef { ty, .. } => ty,
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Definition::ExprDef { name, body, ty, .. } => match ty {
                None => write!(f, "def {name} := {body}"),
                Some(ty) => write!(f, "def {name} : {ty} := {body}"),
            },
        }
    }
}
