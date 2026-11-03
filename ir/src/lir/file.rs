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
    pub items: Vec<Item>,
}
