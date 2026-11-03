use crate::lir::{Expression, Global};
use pp::pretty::*;

// ============================================================================
// Item
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// compute a global
    Global(Global),
    /// compute a value for the user
    Eval(Expression),
}

// ============================================================================
// File
// ============================================================================

/// code of a file, run in order
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct File {
    pub items: Vec<Item>,
}

impl Pretty for Item {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Item::Global(global) => global.pretty(theme),
            Item::Eval(expr) => theme
                .keyword(&"eval")
                .append(Doc::space())
                .append(expr.pretty(theme))
                .group(),
        }
    }
}

impl Pretty for File {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.items.iter().map(|item| item.pretty(theme)),
            Doc::hardline(),
        )
    }
}
