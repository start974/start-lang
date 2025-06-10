use crate::location2::{Loc, Located, Location};

pub struct Identifier(Loc<String>);

impl Identifier {
    /// Creates a new identifier with the given name and location.
    pub fn new(name: String, loc: Location) -> Self {
        Self(Loc::new(name, loc))
    }

    /// name of identifier
    pub fn name(&self) -> &str {
        &self.0.data
    }
}

impl Located for Identifier {
    fn loc(&self) -> &Location {
        &self.0.loc
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.0.data == other.0.data
    }
}

impl Eq for Identifier {}

impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.data.hash(state);
    }
}
