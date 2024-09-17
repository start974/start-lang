use std::fmt;
use crate::utils::colored::Colored;

pub struct Program<TDef> {
    data: Vec<TDef>,
}

impl<TDef> Program<TDef> {
    /// make an empty program
    pub fn empty() -> Self {
        Program { data: Vec::new() }
    }

    /// add definition to program
    /// return [Some definition] if [definition] already exists
    pub fn add_definition(mut self, def: TDef) -> Self {
        self.data.push(def);
        self
    }

    /// iterator over definitions
    pub fn iter(&self) -> impl Iterator<Item = &TDef> {
        self.data.iter()
    }
}

impl<TDef> fmt::Display for Program<TDef>
where
    TDef: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for def in &self.data {
            writeln!(f, "{}", def)?;
        }
        Ok(())
    }
}

impl<TDef> Colored for Program<TDef>
where
    TDef: Colored,
{
    fn colored(&self) -> String {
        let mut res = String::new();
        for def in &self.data {
            res += &def.colored();
            res += "\n";
        }
        res
    }
}

