/*
use super::ast::{
    ty::{Ty, Typed, WeakTyped},
    typed, untyped, Ident, Program,
};

use std::collections::HashMap;

struct TypedEnv {
    data: HashMap<Ident, Ty>,
}

impl TypedEnv {
    /// empty typed environment
    pub fn empty() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// add binding to typed environment
    pub fn add_binding(mut self, ident: Ident, ty: Ty) -> Self {
        self.data.insert(ident, ty);
        self
    }

    fn
}

impl From<Program> for TypedEnv {
    fn from(prog: Program) -> Self {
        let mut typed_env = TypedEnv::empty();
        for def in prog.iter() {
            todo!();
        }
        typed_env
    }
}
*/
