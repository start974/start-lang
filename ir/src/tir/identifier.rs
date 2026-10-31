use std::{collections::HashMap, rc::Rc};

// ==========================================================================
// Identifier
// ==========================================================================

/// unique name of a definition: its name and its rank among the definitions
/// of that name (`a__0`, `a__1`, …)
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Identifier {
    name: String,
    id: usize,
}

impl Identifier {
    /// get name of Identifier
    pub fn name(&self) -> &str {
        &self.name
    }

    /// get id
    pub fn id(&self) -> usize {
        self.id
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", self.name, self.id)
    }
}

// ==========================================================================
// Identifier Builder
// ==========================================================================

#[derive(Debug, Default, Clone)]
pub struct IdentifierBuilder {
    table: HashMap<String, Vec<Rc<Identifier>>>,
}

impl IdentifierBuilder {
    /// create a new identifier
    pub fn build(&mut self, name: &str) -> Rc<Identifier> {
        let idents = self.table.entry(name.to_string()).or_default();
        let ident = Rc::new(Identifier {
            name: name.to_string(),
            id: idents.len(),
        });
        idents.push(Rc::clone(&ident));
        ident
    }

    /// get the last identifier built for `name`, or a fresh one if there is
    /// none (it is then unknown to the environment)
    pub fn get(&self, name: &str) -> Rc<Identifier> {
        self.table
            .get(name)
            .and_then(|idents| idents.last().cloned())
            .unwrap_or_else(|| {
                Rc::new(Identifier {
                    name: name.to_string(),
                    id: 0,
                })
            })
    }
}
