use crate::error::*;
use crate::location::Located;
use crate::parser::ast::*;
use crate::utils::{colored::Colored, FResult};

use super::ast::*;
use super::env::TypingEnv;

pub struct Typer {
    env: TypingEnv,
}

type TypingResult<T, E> = FResult<Typer, T, E>;

const ERROR_TYPE_NOT_FOUND: i32 = 301;
const ERROR_TYPE_MISMATCH: i32 = 302;
const ERROR_VAR_NOT_FOUND: i32 = 303;

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

    fn ok<T, E>(self, val: T) -> TypingResult<T, E> {
        TypingResult::ok(self, val)
    }

    fn error<T, T2>(self, msg: String, elm: &T2, id: i32) -> TypingResult<T, Error>
    where
        T2: Located,
    {
        let err = Error::make(&msg, id).copy_location(elm);
        TypingResult::err(self, err)
    }

    fn get_binding(self, name: &Ident) -> TypingResult<Ty, Error> {
        match self.env.get_binding(name) {
            Some(ty) => self.ok(ty),
            None => {
                let msg = format!("Variable '{name}' not found");
                self.error(msg, name, ERROR_VAR_NOT_FOUND)
            }
        }
    }

    fn assert_ty<T2>(self, ty1: &Ty, elm2: &T2) -> TypingResult<(), Error>
    where
        T2: WeakTyped + Located,
    {
        match elm2.get_opt_ty() {
            Some(ty2) if !self.env.mem(ty2) => {
                let msg = format!("Type '{ty2}' not found");
                self.error(msg, ty2, ERROR_TYPE_NOT_FOUND)
            }
            Some(ty2) if ty1 != ty2 => {
                let msg = format!("Expected type {ty1}, found type {ty2}");
                self.error(msg, elm2, ERROR_TYPE_MISMATCH)
            }
            _ => self.ok(()),
        }
    }
    fn assert_ty2<T1, T2>(self, elm1: &T1, elm2: &T2) -> TypingResult<(), Error>
    where
        T1: Typed,
        T2: WeakTyped + Located,
    {
        let ty1 = elm1.get_ty();
        self.assert_ty(ty1, elm2)
    }

    pub fn type_expression(self, expr: &WTExpression) -> TypingResult<TExpression, Error> {
        match &expr.kind {
            ExpressionKind::Const(constant) => self
                .assert_ty2(constant, expr)
                .map_res(|()| TExpression::make_constant(constant.clone()))
                .map_res(|expr2| expr2.copy_location(expr)),
            ExpressionKind::Var(x) => self
                .get_binding(x)
                .and_then(|typing, ty| {
                    typing
                        .assert_ty(&ty, expr)
                        .map_res(|()| TExpression::make_var(x.clone(), ty))
                })
                .map_res(|expr2| expr2.copy_location(expr)),
        }
    }

    pub fn type_expr_def(self, def: &WTExprDef) -> TypingResult<TDefinition, Error> {
        let name = def.get_name();
        self.type_expression(def.get_body())
            .map_acc2(|typing, body| typing.add_binding(name.clone(), body))
            .and_then(|typing, body| typing.assert_ty2(&body, def).map_res(|()| body))
            .map_res(|body| TDefinition::make_expr_def(name.clone(), body))
            .map_res(|def2| def2.copy_location(def))
    }

    pub fn type_definition(self, def: &WTDefinition) -> TypingResult<TDefinition, Error> {
        match def {
            WTDefinition::ExprDef(expr_def) => self.type_expr_def(expr_def),
            WTDefinition::TyDef(_ty_def) => todo!(),
        }
    }

    /// type a program
    pub fn type_program(self, program: &WTProgram) -> TypingResult<TProgram, Errors> {
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
    ) -> TypingResult<TDefsOrExpr, Errors> {
        match defs_or_exp {
            WTDefsOrExpr::Expression(expr) => self
                .type_expression(expr)
                .map_res(TDefsOrExpr::Expression)
                .to_errors(),
            WTDefsOrExpr::Definitions(prog) => {
                self.type_program(prog).map_res(TDefsOrExpr::Definitions)
            }
        }
    }
}

impl std::fmt::Display for Typer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.env.fmt(f)
    }
}

impl Colored for Typer {
    fn colored(&self) -> String {
        self.env.colored()
    }
}
