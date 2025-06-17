use crate::utils::location::{Loc, Located, Location};

#[derive(Debug)]
pub struct Identifier(Loc<String>);

impl Identifier {
    /// Creates a new identifier with the given name and location.
    pub fn new(name: &str, loc: Location) -> Self {
        Self(Loc::new(name.to_string(), loc))
    }

    /// name of identifier
    pub fn name(&self) -> &str {
        &self.0.data
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Located for Identifier {
    fn loc(&self) -> &Location {
        &self.0.loc
    }
}
