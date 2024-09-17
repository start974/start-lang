use crate::ast::Ident;
use crate::location::Location;
use crate::utils::colored::*;
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

    /// add indetifier to environment
    pub fn add_ident(mut self, ident: Ident) -> Option<Self> {
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
                let name_env = self.add_ident(ident.clone()).unwrap();
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

impl fmt::Display for NameEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in &self.data {
            writeln!(f, "{}\t->\t{}\t{}", k, v.name, v.id)?;
        }
        Ok(())
    }
}

impl Colored for NameEnv {
    fn colored(&self) -> String {
        let mut s = String::new();
        for (k, v) in &self.data {
            s.push_str(&cformat!(
                "<green>{}</>\t->\t<blue>{}</>\t<cyan>{}</>\n",
                k,
                v.name,
                v.id
            ));
        }
        s
    }
}
