use crate::utils::location::{Located, LocatedSet, Location, UNKNOWN_LOCATION};
use std::hash::Hash;

// ==========================================================================
// Identifier
// ==========================================================================

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Name {
    //Fresh,
    Named(String),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    name: Name,
    id: usize,
    loc: Location,
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
        write!(f, "{}_{}", self.name(), self.id())
    }
}

impl Located for Identifier {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

impl LocatedSet for Identifier {
    fn set_loc(&mut self, loc: &impl Located) {
        self.loc = loc.loc().clone();
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.id == other.id
    }
}

impl Eq for Identifier {}

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.id.hash(state);
    }
}

// ==========================================================================
// Indentifier Builder
// ==========================================================================

#[derive(Debug)]
pub struct IdentifierBuilder {
    table: std::collections::HashMap<String, usize>,
    //fresh_id: usize,
    //snapshot: Box<Option<IdentifierBuilder>>,
}

impl IdentifierBuilder {
    /// create a new identifier builder
    pub fn new() -> Self {
        Self {
            table: std::collections::HashMap::new(),
            //fresh_id: 1,
            //snapshot: Box::new(None),
        }
    }

    ///// crate fresh identifier
/*    pub fn fresh(&mut self) -> Identifier {*/
        /*let id = self.fresh_id;*/
        /*self.fresh_id += 1;*/
        /*Identifier {*/
            /*name: Name::Fresh,*/
            /*id,*/
            /*loc: UNKNOWN_LOCATION,*/
        /*}*/
    /*}*/

    fn identifier_id(&self, name: &str) -> Option<usize> {
        self.table
            .get(name)
            .cloned()
/*            .or_else(|| match self.snapshot.as_ref() {*/
                /*Some(builder) => builder.identifier_id(name),*/
                /*None => None,*/
            /*})*/
    }

    /// create a new identifier
    pub fn build(&mut self, name: &str) -> Identifier {
        let id = self.identifier_id(name).unwrap_or(0);
        self.table.insert(name.to_string(), id + 1);
        Identifier {
            name: Name::Named(name.to_string()),
            id,
            loc: UNKNOWN_LOCATION,
        }
    }

    /// get identifier by name
    pub fn get(&self, name: &str) -> Identifier {
        let id = self.identifier_id(name).unwrap_or(0);
        Identifier {
            name: Name::Named(name.to_string()),
            id,
            loc: UNKNOWN_LOCATION,
        }
    }

/*    /// take a snapshot of the current state*/
    /*pub fn snapshot(self) -> Self {*/
        /*Self {*/
            /*table: std::collections::HashMap::new(),*/
            /*//fresh_id: self.fresh_id,*/
            /*snapshot: Box::new(Some(self)),*/
        /*}*/
    /*}*/

    /*/// restore the snapshot*/
    /*pub fn restore(self) -> Self {*/
        /*self.snapshot.expect("No snapshot to restore")*/
    /*}*/
}

impl Default for IdentifierBuilder {
    fn default() -> Self {
        Self::new()
    }
}
