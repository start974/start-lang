use super::super::location::Location;
use super::ast::Ident;
use std::collections::HashMap;
use std::fmt;

pub struct NameEnv {
    data: HashMap<String, Ident>,
}

impl NameEnv {
    /// make
    pub fn empty() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// add idetifier to environment
    pub fn add(mut self, ident: Ident) -> Option<Self> {
        match self.data.insert(ident.name.clone(), ident) {
            Some(_) => None,
            None => Some(self),
        }
    }
    // get a identifier (clone)
    pub fn get(&self, var: &String) -> Option<Ident> {
        self.data.get(var).cloned()
    }

    /// make identifier with variable name
    pub fn of_string(self, name: String) -> (Self, Ident) {
        match self.get(&name) {
            Some(ident) => (self, ident),
            None => {
                let ident = Ident::from(name.clone());
                let name_env = self.add(ident.clone()).unwrap();
                (name_env, ident)
            }
        }
    }

    /// add identifier with location
    pub fn of_location(self, location: &Location) -> (Self, Ident) {
        self.of_string(location.text())
    }

    // get identifier with variable
    //pub fn fresh(&mut self, location: &Option<Location>) -> &Ident {
    //let name = "_x".to_string();
    //self.make_ident(&name, location)
    //}
}

impl Clone for NameEnv {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl fmt::Debug for NameEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data.is_empty() {
            f.write_str("{ }")
        } else {
            f.write_str("{\n")?;
            for (k, v) in &self.data {
                writeln!(f, "  {} -> {}", k, v)?;
            }
            f.write_str("}")
        }
    }
}
