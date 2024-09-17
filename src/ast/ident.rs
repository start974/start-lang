pub use crate::location::{Located, Location};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
pub struct Ident {
    pub name: String,
    location: Option<Location>,
    pub id: u32,
}

impl From<String> for Ident {
    fn from(name: String) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Self {
            name,
            location: None,
            id: COUNTER.fetch_add(1, Ordering::SeqCst) as u32,
        }
    }
}

impl From<&str> for Ident {
    fn from(name: &str) -> Self {
        name.to_string().into()
    }
}

impl From<Location> for Ident {
    fn from(location: Location) -> Self {
        let name = location.text();
        name.into()
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.name, self.id)
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Ident {}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Located for Ident {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        self.location = opt_location;
        self
    }
}
