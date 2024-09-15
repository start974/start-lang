use super::ast::{Ident, Ty};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TypingEnv {
    bindings: HashMap<Ident, Ty>,
    type_set: HashSet<Ty>,
}

impl TypingEnv {
    /// empty typing environment
    pub fn empty() -> Self {
        Self {
            bindings: HashMap::new(),
            type_set: HashSet::new(),
        }
    }

    // add binding to typing environment
    pub fn add_binding(mut self, ident: Ident, ty: Ty) -> Self {
        let _ = self.bindings.insert(ident, ty);
        self
    }

    // add types in type set
    pub fn add_type(mut self, ty: Ty) -> Self {
        let _ = self.type_set.insert(ty);
        self
    }

    // check if type exists in type set
    pub fn mem(&self, ty: &Ty) -> bool {
        self.type_set.contains(ty)
    }
}
