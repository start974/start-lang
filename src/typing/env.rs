use super::ast::{Ident, Ty};
use std::collections::HashMap;

pub struct TypingEnv {
    bindings: HashMap<Ident, Ty>,
}

impl TypingEnv {
    /// empty typing environment
    pub fn empty() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    // add binding to typing environment
    pub fn add_binding(mut self, ident: Ident, ty : Ty) -> Self {
        let _ = self.bindings.insert(ident, ty);
        self
    }
}
