use super::super::super::ast::Identifier;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct RuleName {
    name: Identifier,
    level: Option<u32>,
}

impl RuleName {
    /// make a new rule name
    pub fn new(name: Identifier, level: Option<u32>) -> Self {
        RuleName { name, level }
    }
}

impl Pretty for RuleName {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil().append(
            theme
                .grammar_rule(&self.name)
                .append(if let Some(level) = self.level {
                    Doc::text(format!("[level: {}]", level))
                } else {
                    Doc::nil()
                }),
        )
    }
}
