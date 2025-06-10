use crate::location2::{Loc, Located, Location};

pub struct Identifier<Path>(Loc<Path, String>);

impl<Path> Identifier<Path> {
    /// Creates a new identifier with the given name and location.
    pub fn new(name: String, loc: Location<Path>) -> Self {
        Self(Loc::new(name, loc))
    }

    /// name of identifier
    pub fn name(&self) -> &str {
        &self.0.data
    }
}

impl<Path> Located<Path> for Identifier<Path> {
    fn loc(&self) -> &Location<Path> {
        &self.0.loc
    }
}

impl<Path> PartialEq for Identifier<Path> {
    fn eq(&self, other: &Self) -> bool {
        self.0.data == other.0.data
    }
}

impl<Path> Eq for Identifier<Path> {}

impl<Path> std::hash::Hash for Identifier<Path> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.data.hash(state);
    }
}
