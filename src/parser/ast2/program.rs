use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::{ExpressionDefinition, TyDefinition};

pub enum Item {
    TyDef(TyDefinition),
    ExprDef(ExpressionDefinition),
}
pub struct Program(Vec<Item>);

impl Program {
    /// make an empty program
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// push an item
    pub fn push_item(&mut self, item: Item) {
        self.0.push(item);
    }

    pub fn items(&self) -> &[Item] {
        &self.0
    }
}

impl Pretty for Item {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Item::TyDef(def) => def.pretty(theme),
            Item::ExprDef(def) => def.pretty(theme),
        }
    }
}

impl Pretty for Program {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.items().iter().map(|item| item.pretty(theme)),
            Doc::line(),
        )
    }
}
