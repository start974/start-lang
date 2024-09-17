use super::env::EvalEnv;
use super::value::Value;
use crate::typing::ast::*;

pub struct Context {
    env: EvalEnv,
}

impl Context {
    pub fn empty() -> Self {
        Self {
            env: EvalEnv::empty(),
        }
    }

    pub fn get(&self, ident: &Ident) -> Option<&TExpression> {
        self.env.get(ident)
    }

    pub fn get_main(&self) -> Option<&Ident> {
        self.env.get_main()
    }

    pub fn add_definition(mut self, definition: TDefinition) -> Self {
        self.env = self.env.add_definition(definition);
        self
    }

    pub fn add_program(mut self, program: TProgram) -> Self {
        for def in program.iter() {
            self = self.add_definition(def.clone());
        }
        self
    }

    pub fn eval_const(&self, c: &Constant) -> Value {
        match c {
            Constant::N(n) => Value::N(n.clone()),
        }
    }

    pub fn eval_expr(&self, expr: &TExpression) -> Value {
        match &expr.kind {
            ExpressionKind::Const(c) => self.eval_const(c),
        }
    }
}
