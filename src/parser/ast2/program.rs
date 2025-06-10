use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::{ExpressionDefinition, TyDefinition};

pub enum Item<Path> {
    TyDef(TyDefinition<Path>),
    ExprDef(ExpressionDefinition<Path>),
}
pub struct Program<Path>(Vec<Item<Path>>);

impl<Path> Program<Path> {
    /// make an empty program
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// push an item
    pub fn push_item(&mut self, item: Item<Path>) {
        self.0.push(item);
    }

    pub fn items(&self) -> &[Item<Path>] {
        &self.0
    }
}

impl<Path> Pretty for Item<Path> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Item::TyDef(def) => def.pretty(theme),
            Item::ExprDef(def) => def.pretty(theme),
        }
    }
}

impl<Path> Pretty for Program<Path> {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.items().iter().map(|item| item.pretty(theme)),
            Doc::line(),
        )
    }
}
