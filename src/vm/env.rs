use crate::typing::ast::{Expression, ExpressionDefinition};

use super::identifier::Identifier;
use super::value::{Constant, Value};
use std::collections::HashMap;

pub struct Env(HashMap<Identifier, Value>);

impl Env {
    /// make a new environment
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// get environment value
    pub fn get(&self, id: &Identifier) -> Option<&Value> {
        self.0.get(id)
    }

    /// set environment value
    pub fn set(&mut self, id: Identifier, value: Value) {
        self.0.insert(id, value);
    }

    /// eval expression
    pub fn eval(&self, expr: &Expression) -> Value {
        match expr {
            Expression::Constant(c) => Value::from(Constant::from(c)),
            Expression::Variable(x) => {
                let id_ty = x.identifier();
                let id = Identifier::from(id_ty);
                match self.get(&id) {
                    Some(value) => value.clone(),
                    None => panic!("Variable {} not found", id_ty),
                }
            }
        }
    }

    /// add a definition to the environment
    pub fn add_definition(&mut self, def: &ExpressionDefinition) {
        let id = Identifier::from(def.name());
        let value = self.eval(def.body());
        self.set(id, value);
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
