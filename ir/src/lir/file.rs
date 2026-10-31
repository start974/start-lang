use crate::lir::{Expression, Global};

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
    items: Vec<Item>,
}

impl File {
    /// get items
    pub fn items(&self) -> &[Item] {
        &self.items
    }
}

impl FromIterator<Item> for File {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}
