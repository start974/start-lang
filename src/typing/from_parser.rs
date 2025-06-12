use super::ast as ast_typed;
use crate::parser::ast as ast_parser;
use crate::utils::location::LocatedSet;

/// constant from parser
fn from_const(constant: &ast_parser::Constant) -> ast_typed::Constant {
    match constant.kind() {
        ast_parser::ConstantKind::N(n) => ast_typed::Constant::n(n.clone()),
        ast_parser::ConstantKind::B(b) => ast_typed::Constant::b(*b),
    }
    .with_loc(constant)
}

/*/// type from parser*/
/*fn ty(ty: &ast_parser::Ty) -> ast_typed::Ty {*/
/*match ty.kind() {*/
/*ast_parser::TyKind::N => ast_typed::Ty::n(),*/
/*ast_parser::TyKind::Bool => ast_typed::Ty::bool(),*/
/*ast_parser::TyKind::Var(ident) => ast_typed::Ty::var(ident.clone()),*/
/*}*/
/*.with_loc(ty)*/
/*}*/
