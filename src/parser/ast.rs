pub use crate::ast::pretty_print::*;
pub use crate::ast::*;

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

impl Pretty for WTExprDef {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let doc_ty = self.ty.iter().fold(Doc::nil(), |doc, ty| {
            Doc::group(
                doc.append(theme.op_typed_by())
                    .append(Doc::space())
                    .append(ty.pretty(theme)),
            )
            .append(Doc::space())
        });
        Doc::group(
            Doc::nil()
                .append(theme.kw_def())
                .append(Doc::space())
                .append(theme.def_var(&self.name))
                .append(Doc::space())
                .append(doc_ty)
                .append(theme.op_eq_def())
                .append(Doc::line())
                .append(self.body.pretty(theme)),
        )
    }
}

//-----------------------------------------------------------------------------
// Definition
//-----------------------------------------------------------------------------

impl Pretty for WTDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Self::ExprDef(expr_def) => expr_def.pretty(theme),
            Self::TyDef(ty_def) => ty_def.pretty(theme),
        }
    }
}
