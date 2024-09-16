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

    pub fn add_definition(mut self, definition: TDefinition) -> Self {
        self.env = self.env.add_definition(definition);
        self
    }

    //fn get_definition(&self, ident: &Ident) -> Option<&TExpression> {
    //self.env.get_definition(ident)
    //}

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
