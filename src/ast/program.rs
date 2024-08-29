use super::definition::Definition;
use super::ident::Ident;

use std::collections::HashMap;
use std::fmt;

pub struct Program {
    map: HashMap<Ident, Definition>,
}

impl Program {
    /// make an empty program
    pub fn empty() -> Self {
        Program {
            map: HashMap::new(),
        }
    }

    /// add definition to program
    /// return [Some definition] if [definition] already exists
    pub fn add_definition(mut self, def: Definition) -> (Self, Option<Definition>) {
        let def_opt = self.map.insert(def.get_name().clone(), def);
        (self, def_opt)
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for def in self.map.values() {
            writeln!(f, "{}", def)?
        }
        Ok(())
    }
}
