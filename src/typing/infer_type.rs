use crate::error::*;
use crate::location::Located;
use crate::parser::ast::*;
use crate::utils::FResult;

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

    fn get_binding(self, name: &Ident, location: &Option<Location>) -> TypingResult<Ty, Error> {
        match self.env.get_binding(name) {
            Some(ty) => self.ok(ty),
            None => {
                let msg = Head::new()
                    .text("Variable")
                    .quoted(&name.to_string())
                    .text("not found");
                let err = Error::make(msg, ERROR_VAR_NOT_FOUND).set_opt_location(location.clone());
                TypingResult::err(self, err)
            }
        }
    }

    fn assert_ty<T2>(self, ty1: &Ty, elm2: &T2) -> TypingResult<(), Error>
    where
        T2: WeakTyped + Located,
    {
        match elm2.get_opt_ty() {
            Some(ty2) if !self.env.mem(ty2) => {
                let msg = Head::new()
                    .text("Type")
                    .quoted(&ty2.to_string())
                    .text("not found");
                let err = Error::make(msg, ERROR_TYPE_NOT_FOUND).copy_location(ty2);
                TypingResult::err(self, err)
            }
            Some(ty2) if ty1 != ty2 => {
                let msg = Head::new().text("Type mismatch");
                let err = Error::make(msg, ERROR_TYPE_MISMATCH)
                    .copy_location(elm2)
                    .add_hint(Hint::new().text("Expect type:").quoted(&ty1.to_string()))
                    .add_hint(Hint::new().text("Found type: ").quoted(&ty2.to_string()));
                TypingResult::err(self, err)
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
                .map_res(|()| TExpression::make_constant(constant.clone()).copy_location(expr)),
            ExpressionKind::Var(x) => {
                self.get_binding(x, expr.get_location())
                    .and_then(|typing, ty| {
                        typing
                            .assert_ty(&ty, expr)
                            .map_res(|()| TExpression::make_var(x.clone(), ty).copy_location(expr))
                    })
            }
        }
    }

    pub fn type_expr_def(self, def: &WTExprDef) -> TypingResult<TDefinition, Error> {
        let name = def.get_name();
        self.type_expression(def.get_body())
            .map_acc2(|typing, body| typing.add_binding(name.clone(), body))
            .and_then(|typing, body| {
                typing
                    .assert_ty2(&body, def)
                    .map_res(|()| TDefinition::make_expr_def(name.clone(), body))
            })
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
