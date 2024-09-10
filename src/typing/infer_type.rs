use super::super::error::Error;
use super::super::location::Located;
use super::super::parser::ast::*;
use super::super::stdlib::TYPE_ENV;
use super::ast::*;
use super::env::TypingEnv;

pub struct Typer {
    env: TypingEnv,
}

type TypingResult<T> = (Typer, Result<T, Error>);

impl Typer {
    pub fn make() -> Self {
        Self {
            env: TYPE_ENV.clone(),
        }
    }

    fn add_binding<T>(mut self, name: Ident, elm: &T) -> Self
    where
        T: Typed,
    {
        self.env = self.env.add_binding(name, elm.get_ty().clone());
        self
    }

    fn assert_ty<T1, T2>(&self, elm1: &T1, elm2: &T2) -> Result<(), Error>
    where
        T1: Typed,
        T2: WeakTyped + Located,
    {
        match elm2.get_opt_ty() {
            Some(ty2) if !self.env.mem(ty2) => {
                let msg = format!("Type '{ty2}' no exists");
                match ty2.get_location() {
                    None => panic!("{msg}"),
                    Some(location) => Err(Error::error_located(&msg, location.clone())),
                }
            }
            Some(ty2) if elm1.get_ty() != ty2 => {
                let msg = "Expected type {ty1}, found type {ty2}";
                match (ty2.get_location(), elm2.get_location()) {
                    (None, None) => panic!("{msg}"),
                    (_, Some(location)) | (Some(location), _) => {
                        Err(Error::error_located(msg, location.clone()))
                    }
                }
            }
            _ => Ok(()),
        }
    }

    pub fn type_expression(self, expr: &WTExpression) -> TypingResult<TExpression> {
        match &expr.kind {
            ExpressionKind::Const(constant) => {
                let res_expr = self
                    .assert_ty(constant, expr)
                    .map(|()| expr.get_location().clone())
                    .map(|location| {
                        TExpression::make_constant(constant.clone()).set_opt_location(location)
                    });
                (self, res_expr)
            }
        }
    }

    pub fn type_expr_def(self, def: &WTExprDef) -> TypingResult<TDefinition> {
        let (typing, res_body) = self.type_expression(def.get_body());
        match res_body {
            Ok(body) => {
                let typing = typing.add_binding(def.get_name().clone(), &body);
                let res_def = typing.assert_ty(&body, def).map(|()| {
                    TDefinition::make_expr_def(def.get_name().clone(), body)
                        .set_opt_location(def.get_location().clone())
                });
                (typing, res_def)
            }
            Err(err) => (typing, Err(err)),
        }
    }

    pub fn type_definition(self, def: &WTDefinition) -> TypingResult<TDefinition> {
        match def {
            WTDefinition::ExprDef(expr_def) => self.type_expr_def(expr_def),
        }
    }

    /// type a program
    pub fn type_program(self, program: &WTProgram) -> TypingResult<TProgram> {
        program.iter().fold(
            (self, Ok(Program::empty())),
            |(typing, mut res_prog), def| {
                let (typing, res_def) = typing.type_definition(def);
                res_prog = match (res_prog, res_def) {
                    (Ok(program_typed), Ok(def_typed)) => {
                        Ok(program_typed.add_definition(def_typed))
                    }
                    (Err(err), Ok(_)) | (Ok(_), Err(err)) => Err(err),
                    (Err(err1), Err(err2)) => Err(err1.error_add(err2)),
                };
                (typing, res_prog)
            },
        )
    }
}
