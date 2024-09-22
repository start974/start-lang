use super::value::*;
use crate::error::*;
use crate::stdlib::number_n::N_TYPE;
use crate::typing::ast::*;
use crate::typing::ast::{TDefinition, TExpression};
use crate::utils::colored::*;
use std::collections::HashMap;
use std::sync::LazyLock;

pub struct Interpreter {
    env: HashMap<Ident, Value>,
    main: Option<DefValue>,
}

static MAIN_TY: LazyLock<Ty> = LazyLock::new(|| N_TYPE.clone());
const ERROR_MAIN_NOT_FOUND: i32 = 401;
const ERROR_MAIN_TYPE: i32 = 402;

impl Interpreter {
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
    pub fn add_program(mut self, program: &TProgram) -> (Self, DefValues) {
        let mut def_vals = DefValues::new();
        for def in program.iter() {
            let (interpreter, def_val) = self.add_definition(def);
            def_vals.push(def_val);
            self = interpreter;
        }
        (self, def_vals)
    }

    /// evaluate expression or definitions
    pub fn eval_definitions_or_expression(self, defs_or_expr: &TDefsOrExpr) -> (Self, DefsOrValue) {
        match defs_or_expr {
            TDefsOrExpr::Definitions(prog) => {
                let (interpreter, defs) = self.add_program(prog);
                (interpreter, DefsOrValue::Defs(defs))
            }
            TDefsOrExpr::Expression(expr) => {
                let value = self.eval_expr(expr);
                (self, DefsOrValue::Value(value))
            }
        }
    }

    /// evaluation main
    pub fn eval_main(&self) -> Result<Value, Error> {
        match &self.main {
            None => {
                let msg = Head::new()
                    .text("Function")
                    .quoted("main")
                    .text("not found");
                Err(Error::make(msg, ERROR_MAIN_NOT_FOUND))
            }
            Some(main) if main.ty != *MAIN_TY => {
                let msg = Head::new()
                    .text("Function")
                    .quoted("main")
                    .text("has wrong type");
                let err = Error::make(msg, ERROR_MAIN_TYPE)
                    .copy_location(main)
                    .add_hint(Hint::new().text("Expect :").quoted(&MAIN_TY.to_string()))
                    .add_hint(Hint::new().text("Got    :").quoted(&main.ty.to_string()));
                Err(err)
            }
            Some(main) => Ok(main.value.clone()),
        }
    }
}

impl std::fmt::Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.env {
            writeln!(f, "{}\n{}", k, v)?;
        }
        match &self.main {
            None => writeln!(f, "main : ⊥"),
            Some(main) => writeln!(f, "main :\n{}", main),
        }
    }
}

impl Colored for Interpreter {
    fn colored(&self) -> String {
        let mut s = String::new();
        for (k, v) in &self.env {
            s += &cformat!("<blue>{}:</>\n{}\n", k, v.colored());
        }
        match &self.main {
            None => s += &cformat!("<bold>main :</> ⊥"),
            Some(main) => s += &cformat!("<bold>main</>:\n{}", main.colored()),
        }
        s
    }
}
