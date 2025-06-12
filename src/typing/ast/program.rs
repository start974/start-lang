use super::expression::ExpressionDefinition;
use super::identifier::Identifier;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};
use std::collections::HashMap;
use std::rc::Rc;

// =======================================================================
// Program
// =======================================================================

pub struct Program {
    env: HashMap<Rc<Identifier>, ExpressionDefinition>,
    ordered_env: Vec<Rc<Identifier>>,
}

impl Program {
    /// make program
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            ordered_env: Vec::new(),
        }
    }

    /// push a new definition
    /// return none if definition already exist
    pub fn with_definition(mut self, def: ExpressionDefinition) -> Option<Self> {
        let identifier = Rc::new(def.name().clone());
        match self.env.insert(identifier.clone(), def) {
            None => {
                self.ordered_env.push(identifier);
                Some(self)
            }
            Some(_) => None,
        }
    }

    /// iter on environment (without main)
    pub fn iter(&self) -> impl Iterator<Item = &ExpressionDefinition> {
        self.ordered_env.iter().map(|id| self.env.get(id).unwrap())
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Pretty for Program {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.iter().map(|item| item.pretty(theme)),
            Doc::line_().append(Doc::line_()),
        )
    }
}
