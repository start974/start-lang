use super::definition::Definition;
use super::ident::Ident;

use std::collections::HashMap;

pub struct Program {
    map: HashMap<Ident, Definition>,
}

impl Program {
    /// make an empty program
    pub fn new() -> Self {
        let map: HashMap<Ident, Definition> = HashMap::new();
        Program { map }
    }

    /// add definition to program
    /// return [Some definition] if [definition] already exists
    pub fn add_definition(&mut self, def: Definition) -> Option<Definition> {
        self.map.insert(def.get_ident().clone(), def)
    }
}
