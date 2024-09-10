use super::super::ast::*;
use super::super::typing::ast::{TDefinition, TExpression};
use std::collections::HashMap;

#[derive(Debug)]
pub struct EvalEnv {
    definitions: HashMap<Ident, TExpression>,
}

impl EvalEnv {
    // make empty environment
    pub fn empty() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    // add definition to environment
    pub fn add_definition(mut self, definition: TDefinition) -> Self {
        let name = definition.get_name().clone();
        let body = definition.get_body().clone();
        let _ = self.definitions.insert(name, body);
        self
    }

    // get definition to environment
    //pub fn get_definition(&self, ident: &Ident) -> Option<&TExpression> {
        //self.definitions.get(ident)
    //}
}
