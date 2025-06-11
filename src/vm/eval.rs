use super::value::*;
use crate::error::*;
use crate::typing::ast::*;
use crate::typing::main_function_not_found;
use std::collections::HashMap;

pub struct Interpreter {
    env: HashMap<Ident, Value>,
    main: Option<DefValue>,
}

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
    pub fn eval_main(&self) -> Result<Value, ErrorBox> {
        match &self.main {
            None => Err(main_function_not_found()),
            Some(main) => Ok(main.value.clone()),
        }
    }
}
