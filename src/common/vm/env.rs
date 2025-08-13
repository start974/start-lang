use crate::typer::ast::{Expression, ExpressionDefinition};

use super::identifier::Identifier;
use super::value::{Constant, Value};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Env(HashMap<Identifier, Value>);

impl Env {
    /// get environment value
    pub fn get(&self, id: &Identifier) -> Option<&Value> {
        self.0.get(id)
    }

    /// set environment value
    pub fn set(&mut self, id: Identifier, value: Value) {
        self.0.insert(id, value);
    }

    /// eval expression
    pub fn eval(&self, expr: &Expression) -> Option<Value> {
        match expr {
            Expression::Constant(c) => Some(Value::from(Constant::from(c))),
            Expression::Variable(x) => {
                let id_ty = x.identifier();
                let id = Identifier::from(id_ty);
                self.get(&id).cloned()
            }
        }
    }

    /// add a definition to the environment
    pub fn add_definition(&mut self, def: &ExpressionDefinition) {
        let id = Identifier::from(def.name());
        let value = self.eval(def.body()).unwrap();
        self.set(id, value.clone());
    }
}
