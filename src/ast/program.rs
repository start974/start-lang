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
        let map: HashMap<Ident, Definition> = HashMap::new();
        Program { map }
    }

    /// add definition to program
    /// return [Some definition] if [definition] already exists
    pub fn add_definition(&mut self, def: Definition) -> Option<Definition> {
        let name = def.get_name();
        self.map.insert(name.clone(), def)
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
