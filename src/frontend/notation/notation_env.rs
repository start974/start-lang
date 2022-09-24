use core::fmt;
use std::collections::HashMap;

use super::Notation;
use super::notation_rule::NotationRule;
use super::notation_group::NotationGroup;


pub struct NotationEnv {
    rules: HashMap<String, NotationRule>,
    groups: HashMap<String, NotationGroup>,
}


impl NotationEnv {
    pub fn empty() -> Self {
        NotationEnv {
            rules: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: & NotationRule) {
        match self.rules.insert(rule.name().clone(),rule.to_owned()) {
            None => {
                let group = NotationGroup {
                    name: rule.name().clone(),
                    rules: vec![Box::new(rule.to_owned())],
                };
                self.add_group(&group)
            }
            Some(_) => (),
        };
    }

    pub fn add_group(&mut self, group: &NotationGroup) {
        for rule in group.rules().iter() {
            self.groups.remove(&rule.name());
        }
        self.groups.insert(group.name().clone(), group.to_owned());
    }

    pub fn add_notation(&mut self, notation: &Notation) {
        match notation {
            Notation::Rule(rule) => self.add_rule(rule),
            Notation::Group(group) => self.add_group(group),
        }
    }
}

impl<'a> fmt::Display for NotationEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for groups in self.groups.values() {
            write!(f, "{groups}\n")?;
        }
        Ok(())
    }
}