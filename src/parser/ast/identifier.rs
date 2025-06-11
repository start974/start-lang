use crate::utils::location::{Loc, Located, Location};

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

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.name().to_string()
    }
}

impl Located for Identifier {
    fn loc(&self) -> &Location {
        &self.0.loc
    }
}
