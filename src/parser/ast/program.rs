use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

use super::{ExpressionDefinition, TyDefinition};

pub enum ProgramItem {
    TyDef(TyDefinition),
    ExprDef(ExpressionDefinition),
}
pub struct Program(Vec<ProgramItem>);

impl Program {
    /// make an empty program
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// add an item
    pub fn add_item(&mut self, item: ProgramItem) {
        self.0.push(item);
    }

    /// with item program
    pub fn with_item(mut self, item: ProgramItem) -> Self {
        self.add_item(item);
        self
    }

    pub fn items(&self) -> &[ProgramItem] {
        &self.0
    }
}

impl Pretty for ProgramItem {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            ProgramItem::TyDef(def) => def.pretty(theme),
            ProgramItem::ExprDef(def) => def.pretty(theme),
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
