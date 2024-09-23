pub use super::pretty_print::*;

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

impl<TDef> Pretty for Program<TDef>
where
    TDef: Pretty
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let mut doc = Doc::nil();
        for def in &self.data {
            doc = doc.append(def.pretty(theme)).append(Doc::line())
        }
        doc
    }
}
