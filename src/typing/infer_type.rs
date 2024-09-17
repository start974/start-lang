use crate::error::Error;
use crate::location::Located;
use crate::parser::ast::*;
use crate::utils::FResult;

use super::ast::*;
use super::env::TypingEnv;

pub struct Typer {
    env: TypingEnv,
}

type TypingResult<T> = FResult<Typer, T>;

const ERROR_TYPE_NOT_FOUND: i32 = 301;
const ERROR_TYPE_MISMATCH: i32 = 302;

impl Typer {
    pub fn make(env: TypingEnv) -> Self {
        Self { env }
    }

    fn add_binding<T>(mut self, name: Ident, elm: &T) -> Self
    where
        T: Typed,
    {
        self.env = self.env.add_binding(name, elm.get_ty().clone());
        self
    }

    fn ok<T>(self, val: T) -> TypingResult<T> {
        TypingResult::ok(self, val)
    }

    fn error<T>(self, err: Error) -> TypingResult<T> {
        TypingResult::error(self, err)
    }

    fn assert_ty<T1, T2>(self, elm1: &T1, elm2: &T2) -> TypingResult<()>
    where
        T1: Typed,
        T2: WeakTyped + Located,
    {
        match elm2.get_opt_ty() {
            Some(ty2) if !self.env.mem(ty2) => {
                let msg = format!("Type '{ty2}' not found");
                match ty2.get_location() {
                    None => panic!("{}", Error::error_simple(&msg, ERROR_TYPE_NOT_FOUND)),
                    Some(location) => {
                        let location = location.clone();
                        let err = Error::error_located(&msg, location, ERROR_TYPE_NOT_FOUND);
                        self.error(err)
                    }
                }
            }
            Some(ty2) if elm1.get_ty() != ty2 => {
                let msg = "Expected type {ty1}, found type {ty2}";
                match (ty2.get_location(), elm2.get_location()) {
                    (None, None) => panic!("{}", Error::error_simple(msg, ERROR_TYPE_MISMATCH)),
                    (_, Some(location)) | (Some(location), _) => {
                        let location = location.clone();
                        let err = Error::error_located(msg, location, ERROR_TYPE_MISMATCH);
                        self.error(err)
                    }
                }
            }
            _ => self.ok(()),
        }
    }

    pub fn type_expression(self, expr: &WTExpression) -> TypingResult<TExpression> {
        match &expr.kind {
            ExpressionKind::Const(constant) => self
                .assert_ty(constant, expr)
                .map_res(|()| TExpression::make_constant(constant.clone()))
                .map_res(|expr2| expr2.copy_location(expr)),
            ExpressionKind::Var(_x) => todo!(),
        }
    }

    pub fn type_expr_def(self, def: &WTExprDef) -> TypingResult<TDefinition> {
        let name = def.get_name();
        self.type_expression(def.get_body())
            .map_acc2(|typing, body| typing.add_binding(name.clone(), body))
            .and_then(|typing, body| typing.assert_ty(&body, def).map_res(|()| body))
            .map_res(|body| TDefinition::make_expr_def(name.clone(), body))
            .map_res(|def2| def2.copy_location(def))
    }

    pub fn type_definition(self, def: &WTDefinition) -> TypingResult<TDefinition> {
        match def {
            WTDefinition::ExprDef(expr_def) => self.type_expr_def(expr_def),
        }
    }

    /// type a program
    pub fn type_program(self, program: &WTProgram) -> TypingResult<TProgram> {
        program.iter().fold(self.ok(Program::empty()), |res, def| {
            res.combine(
                |typing| typing.type_definition(def),
                Program::add_definition,
            )
        })
    }

    /// type definitions or expression
    pub fn type_definitions_or_expression(
        self,
        defs_or_exp: &WTDefsOrExpr,
    ) -> TypingResult<TDefsOrExpr> {
        match defs_or_exp {
            WTDefsOrExpr::Expression(expr) => {
                self.type_expression(expr).map_res(TDefsOrExpr::Expression)
            }
            WTDefsOrExpr::Definitions(prog) => {
                self.type_program(prog).map_res(TDefsOrExpr::Definitions)
            }
        }
    }
}
