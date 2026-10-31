use std::{collections::HashMap, hash::Hash, rc::Rc};

// ==========================================================================
// Identifier
// ==========================================================================

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Name {
    //Fresh,
    Named(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Identifier {
    name: Name,
    id: usize,
}

impl Identifier {
    /// get name of Identifier
    pub fn name(&self) -> &str {
        match &self.name {
            //Name::Fresh => &"__fresh__",
            Name::Named(s) => s,
        }
    }

    /// get id
    pub fn id(&self) -> usize {
        self.id
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", self.name(), self.id())
    }
}

// ==========================================================================
// Indentifier Builder
// ==========================================================================

#[derive(Debug, Default)]
pub struct IdentifierBuilder {
    table: HashMap<String, Vec<Rc<Identifier>>>,
}

impl IdentifierBuilder {
    /// create a new identifier
    pub fn build(&mut self, name: &str) -> Rc<Identifier> {
        match self.table.get_mut(name) {
            Some(idents) => {
                let id = idents.len();
                let ident = Identifier {
                    name: Name::Named(name.to_string()),
                    id,
                };
                let ident_rc = Rc::new(ident);
                idents.push(ident_rc.clone());
                ident_rc
            }
            None => {
                let ident = Identifier {
                    name: Name::Named(name.to_string()),
                    id: 0,
                };
                let ident_rc = Rc::new(ident);
                self.table.insert(name.to_string(), vec![ident_rc.clone()]);
                ident_rc
            }
        }
    }

    /// get identifier by name
    pub fn get(&self, name: &str) -> Rc<Identifier> {
        self.table
            .get(name)
            .and_then(|idents| idents.last().cloned())
            .unwrap_or_else(|| {
                let ident = Identifier {
                    name: Name::Named(name.to_string()),
                    id: 0,
                };
                Rc::new(ident)
            })
    }
}
