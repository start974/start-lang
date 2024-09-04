use super::super::error::Error;
use super::super::location::Located;
use super::super::parser::ast::*;
use super::ast::*;
use super::env::TypingEnv;

pub struct Typer {
    env: TypingEnv,
}

type TypingResult<T> = Result<(Typer, T), Error>;

impl Typer {
    pub fn new() -> Self {
        Self {
            env: TypingEnv::empty(),
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
            Some(ty2) if ty1 == ty2 => Ok(()),
            None => Ok(()),
            Some(ty2) => {
                let msg = format!("Expected type {}, found type {}", ty1, ty2);
                match elm.get_location() {
                    None => panic!("{msg}"),
                    Some(location) => Err(Error::error_located(&msg, location.clone())),
                }
            }
        }
    }

    pub fn type_expression(self, expr: &WTExpression) -> TypingResult<TExpression> {
        match &expr.kind {
            ExpressionKind::Const(constant) => {
                let ty = constant.get_ty();
                self.assert_ty(ty, expr)?;
                let expr_ty = TExpression::make_constant(constant.clone(), ty.clone())
                    .set_opt_location(expr.get_location().clone());
                Ok((self, expr_ty))
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
                let (typing, body) = self.type_expression(body)?;
                let ty = body.get_ty();
                typing.assert_ty(ty, def)?;
                let typing = typing.add_binding(name.clone(), ty.clone());
                let def_ty = TDefinition::make_expr_def(name.clone(), ty.clone(), body)
                    .set_opt_location(location.clone());
                Ok((typing, def_ty))
            }
        }
    }

    /// type a program
    pub fn type_program(self, program: &WTProgram) -> TypingResult<TProgram> {
        program
            .iter()
            .try_fold((self, Program::empty()), |(typing, mut program), def| {
                let (typing, def_ty) = typing.type_definition(def)?;
                program = program.add_definition(def_ty);
                Ok((typing, program))
            })
    }
}
