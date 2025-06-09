pub use crate::utils::pretty::*;
pub use crate::ast::*;
use crate::stdlib;
use crate::utils::theme::{Doc, Theme};

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
    pub fn make_expr_def(name: Ident, ty: Ty, body: TExpression) -> Self {
        Self {
            name,
            ty,
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

impl Pretty for TDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::group(
            Doc::nil()
                .append(theme.kw_def())
                .append(Doc::space())
                .append(theme.def_var(&self.name))
                .append(Doc::space())
                .append(
                    Doc::group(theme.op_typed_by())
                        .append(Doc::line())
                        .append(self.ty.pretty(theme)),
                )
                .append(Doc::space())
                .append(theme.op_eq_def())
                .append(Doc::line())
                .append(self.body.pretty(theme)),
        )
    }
}
