use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

// TODO add location
pub struct Ident {
    pub name: String,
    id: u32,
}

impl Ident {
    fn make(name: &String, id: u32) -> Self {
        Ident {
            name: name.clone(),
            id,
        }
    }

    /// ident to string
    pub fn to_string(&self) -> String {
        self.name.clone()
    }

    /// ident debug string (with id)
    pub fn debug_string(&self) -> String {
        format!("{}_{}", self.name, self.id)
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
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
        }
    }
}

struct Env {
    counter: u32,
    map: HashMap<String, Ident>,
}

impl Env {
    /// make
    pub fn empty() -> Self {
        let map: HashMap<String, Ident> = HashMap::new();
        let counter = 0;
        Env { counter, map }
    }

    /// make variable
    pub fn make_var(&mut self, var: &String) -> &Ident {
        let id = self.counter;
        self.counter += 1;
        let ident = Ident::make(var, id);
        self.map.insert(var.clone(), ident);
        self.map.get(var).unwrap()
    }

    /// get identifier with variable
    pub fn fresh(&mut self) -> &Ident {
        let var = "_x".to_string();
        self.make_var(&var)
    }

    /// get variable
    pub fn get_var(&self, var: &String) -> Option<&Ident> {
        self.map.get(var)
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.map.is_empty() {
            f.write_str("{ }")
        } else {
            f.write_str("{\n");
            for (k, v) in &self.map {
                write!(f, "  {} -> {}\n", k, v);
            }
            f.write_str("}")
        }
    }
}
