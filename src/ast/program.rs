use super::definition::Definition;
use super::ident::Ident;

use std::collections::HashMap;
use std::fmt;

pub struct Program<TyT> {
    map: HashMap<Ident, Definition<TyT>>,
}

impl<TyT> Program<TyT> {
    /// make an empty program
    pub fn empty() -> Self {
        Program {
            map: HashMap::new(),
        }
    }

    /// add definition to program
    /// return [Some definition] if [definition] already exists
    pub fn add_definition(mut self, def: Definition<TyT>) -> (Self, Option<Definition<TyT>>) {
        let def_opt = self.map.insert(def.get_name().clone(), def);
        (self, def_opt)
    }
}

impl<TyT> fmt::Display for Program<TyT>
where
    Definition<TyT>: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for def in self.map.values() {
            writeln!(f, "{}", def)?
        }
        Ok(())
    }
}
