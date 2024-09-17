use super::value::*;
use crate::error::Error;
use crate::stdlib::number_n::N_TYPE;
use crate::typing::ast::*;
use crate::typing::ast::{TDefinition, TExpression};
use std::collections::HashMap;
use std::sync::LazyLock;

pub struct Context {
    env: HashMap<Ident, Value>,
    main: Option<DefValue>,
}

static MAIN_TY: LazyLock<Ty> = LazyLock::new(|| N_TYPE.clone());
const ERROR_MAIN_NOT_FOUND: i32 = 401;
const ERROR_MAIN_TYPE: i32 = 402;

impl Context {
    /// empty context
    pub fn empty() -> Self {
        Self {
            env: HashMap::new(),
            main: None,
        }
    }

    /// get definition value
    pub fn get(&self, ident: &Ident) -> Value {
        self.env.get(ident).unwrap().clone()
    }

    /// evaluate constant
    fn eval_const(&self, c: &Constant) -> Value {
        match c {
            Constant::N(n) => Value::N(n.clone()),
        }
    }

    /// evaluate expression
    pub fn eval_expr(&self, expr: &TExpression) -> Value {
        match &expr.kind {
            ExpressionKind::Const(c) => self.eval_const(c),
            ExpressionKind::Var(x) => self.get(x),
        }
    }

    /// add defintion in context
    pub fn add_definition(mut self, definition: &TDefinition) -> (Self, DefValue) {
        let body = definition.body.clone();
        let value = self.eval_expr(&body);
        self.env.insert(definition.name.clone(), value.clone());
        let def_value = DefValue {
            name: definition.name.clone(),
            ty: definition.ty.clone(),
            location: definition.location.clone(),
            value,
        };
        if definition.name.name == "main" {
            self.main = Some(def_value.clone());
        }
        (self, def_value)
    }

    /// add program in context
    pub fn add_program(mut self, program: TProgram) -> Self {
        for def in program.iter() {
            self = self.add_definition(def).0;
        }
        self
    }

    /// evaluation main
    pub fn eval_main(&self) -> Result<i32, Error> {
        match &self.main {
            None => Err(Error::error_simple(
                "main function not found",
                ERROR_MAIN_NOT_FOUND,
            )),
            Some(main) if main.ty != *MAIN_TY => {
                let msg = format!("main function must be typed by '{}' type", *MAIN_TY);
                Err(Error::error_located(
                    &msg,
                    main.location.clone().unwrap(),
                    ERROR_MAIN_TYPE,
                ))
            }
            Some(main) => match &main.value {
                Value::N(v) => Ok(v.try_into().unwrap()),
            },
        }
    }
}
