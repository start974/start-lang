use crate::ast::*;
use crate::typing::ast::{TDefinition, TExpression};
use std::collections::HashMap;

#[derive(Debug)]
pub struct EvalEnv {
    definitions: HashMap<Ident, TExpression>,
    main: Option<Ident>,
}

impl EvalEnv {
    // make empty environment
    pub fn empty() -> Self {
        Self {
            definitions: HashMap::new(),
            main: None,
        }
    }

    // add definition to environment
    pub fn add_definition(mut self, definition: TDefinition) -> Self {
        let name = definition.get_name().clone();
        if name.name == "main" {
            self.main = Some(name.clone());
        }
        let body = definition.get_body().clone();
        let _ = self.definitions.insert(name, body);
        self
    }

    // get definition to environment
    pub fn get(&self, ident: &Ident) -> Option<&TExpression> {
        self.definitions.get(ident)
    }

    pub fn get_main(&self) -> Option<&Ident> {
        self.main.as_ref()
    }

}
