use super::ast::{Ident, Ty};
use crate::utils::colored::*;
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

    // get type of binding
    pub fn get_binding(&self, ident: &Ident) -> Option<Ty> {
        self.bindings.get(ident).cloned()
    }
}

impl std::fmt::Display for TypingEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Type set:")?;
        for ty in &self.type_set {
            writeln!(f, "- {ty}")?;
        }
        writeln!(f, "Bindings :")?;
        for (ident, ty) in &self.bindings {
            writeln!(f, "{ident}\t:\t{ty}")?;
        }
        Ok(())
    }
}

impl Colored for TypingEnv {
    fn colored(&self) -> String {
        let mut s = String::new();
        s += &cformat!("<bold>Type set :</>\n");
        for ty in &self.type_set {
            s += &cformat!("- {}\n", ty.colored());
        }
        s += &cformat!("<bold>Bindings :</>\n");
        for (ident, ty) in &self.bindings {
            s += &cformat!("<blue>{}</>\t:\t{}\n", ident, ty.colored());
        }
        s
    }
}
