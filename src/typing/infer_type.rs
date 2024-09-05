use super::super::error::Error;
use super::super::location::Located;
use super::super::parser::ast::*;
use super::ast::*;
use super::env::TypingEnv;
use super::super::stdlib::TYPE_ENV;

pub struct Typer {
    env: TypingEnv,
}

type TypingResult<T> = (Typer, Result<T, Error>);

impl Typer {
    pub fn new() -> Self {
        Self {
            env: TYPE_ENV.clone(),
        }
    }

    fn add_binding(mut self, name: Ident, ty: Ty) -> Self {
        self.env = self.env.add_binding(name, ty);
        self
    }

    fn assert_ty<T>(&self, ty1: &Ty, elm: &T) -> Result<(), Error>
    where
        T: WeakTyped + Located,
    {
        match elm.get_opt_ty() {
            Some(ty2) if !self.env.mem(ty2) => {
                let msg = format!("Type '{ty2}' no exists");
                match ty2.get_location() {
                    None => panic!("{msg}"),
                    Some(location) => Err(Error::error_located(&msg, location.clone())),
                }
            }
            Some(ty2) if ty1 != ty2 => {
                let msg = "Expected type {ty1}, found type {ty2}";
                match (ty2.get_location(), elm.get_location()) {
                    (None, None) => panic!("{msg}"),
                    (_, Some(location))
                    | (Some(location), _) => Err(Error::error_located(msg, location.clone())),
                }
            }
            _ => Ok(())
        }
    }

    pub fn type_expression(self, expr: &WTExpression) -> TypingResult<TExpression> {
        match &expr.kind {
            ExpressionKind::Const(constant) => {
                let ty = constant.get_ty();
                let res_expr =
                    self.assert_ty(ty, expr)
                        .map(|()| expr.get_location().clone())
                        .map(|location|
                        TExpression::make_constant(constant.clone(), ty.clone()
                            .set_opt_location(location))
                );
                (self, res_expr)
            }
        }
    }

    pub fn type_definition(self, def: &WTDefinition) -> TypingResult<TDefinition> {
        match def {
            WTDefinition::ExprDef {
                name,
                body,
                location,
                ..
            } => {
                let (typing, res_body) = self.type_expression(body);
                match res_body {
                    Ok(body) => {
                        let ty = body.get_ty().clone();
                        let typing = typing.add_binding(name.clone(), ty.clone());
                        let res_def = typing.assert_ty(&ty, def).map(|()|
                            TDefinition::make_expr_def(name.clone(), ty, body)
                    .set_opt_location(location.clone())
                        );
                        (typing, res_def)
                    }
                    Err(err) => (typing, Err(err)),
                }
            }
        }
    }

    /// type a program
    pub fn type_program(self, program: &WTProgram) -> TypingResult<TProgram> {
        program
            .iter()
            .fold((self, Ok(Program::empty())), |(typing, mut res_prog), def| {
                let (typing, res_def) = typing.type_definition(def);
                res_prog = match (res_prog, res_def) {
                    (Ok(program_typed), Ok(def_typed)) => Ok(program_typed.add_definition(def_typed)),
                    (Err(err), Ok(_)) | (Ok(_), Err(err)) => Err(err),
                    (Err(err1), Err(err2)) => Err(err1.error_add(err2)),
                };
                (typing, res_prog)
            })
    }
}
