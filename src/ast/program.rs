use super::definition::Definition;
use super::ident::Ident;
use super::ty::Ty;

use std::fmt;

pub struct Program<TyT> {
    data: Vec<Definition<TyT>>,
}

impl<TyT> Program<TyT> {
    /// make an empty program
    pub fn empty() -> Self {
        Program { data: Vec::new() }
    }

    /// add definition to program
    /// return [Some definition] if [definition] already exists
    pub fn add_definition(mut self, def: Definition<TyT>) -> Self {
        self.data.push(def);
        self
    }

    /// len of program
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// iterator over definitions
    pub fn iter(&self) -> impl Iterator<Item = &Definition<TyT>> {
        self.data.iter()
    }
}

impl<TyT> fmt::Display for Program<TyT>
where
    Definition<TyT>: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for def in &self.data {
            writeln!(f, "{}", def)?;
        }
        Ok(())
    }
}
