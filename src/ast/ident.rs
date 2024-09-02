use super::super::location::{Located, Location};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

// TODO add location
pub struct Ident {
    pub name: String,
    location: Option<Location>,
    id: u32,
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

impl Clone for Ident {
    fn clone(&self) -> Self {
        Ident {
            name: self.name.clone(),
            id: self.id,
            location: self.location.clone(),
        }
    }
}

impl Located for Ident {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }
}

pub struct Env {
    counter: u32,
    map: HashMap<String, Ident>,
}

impl Env {
    /// make
    pub fn empty() -> Self {
        Env {
            counter: 0,
            map: HashMap::new(),
        }
    }

    /// make identifier with variable name
    pub fn of_string(mut self, name: String) -> (Self, Ident) {
        let id = self.counter;
        self.counter += 1;
        let ident = Ident {
            name: name.clone(),
            id,
            location: None,
        };
        let _ = self.map.insert(name.clone(), ident);
        let ident = self.map.get(&name).unwrap().clone();
        (self, ident)
    }

    /// add identifier with location
    pub fn of_location(self, location: &Location) -> (Self, Ident) {
        let name = location.text();
        self.of_string(name)
    }

    // get identifier with variable
    //pub fn fresh(&mut self, location: &Option<Location>) -> &Ident {
    //let name = "_x".to_string();
    //self.make_ident(&name, location)
    //}

    // get variable
    //pub fn get_var(&self, var: &String) -> Option<&Ident> {
    //self.map.get(var)
    //}
}

impl fmt::Debug for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.map.is_empty() {
            f.write_str("{ }")
        } else {
            f.write_str("{\n")?;
            for (k, v) in &self.map {
                writeln!(f, "  {} -> {}", k, v)?;
            }
            f.write_str("}")
        }
    }
}
