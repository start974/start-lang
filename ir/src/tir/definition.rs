use crate::tir::{DefId, DefKind, Documentation, Type};
use std::fmt;

// ============================================================================
// Identifier
// ============================================================================

/// a definition in `tir`: its name, and the `DefId` given by the lowering
/// (shown `a__3`)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
    pub id: DefId,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}__{}", self.name, self.id.0)
    }
}

// ============================================================================
// Definition Information
// ============================================================================

/// what is known about a definition once typed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefInfo {
    pub id: Identifier,
    pub kind: DefKind,
    /// type of an expression, or the type a type definition stands for
    pub ty: Type,
    pub doc: Option<Documentation>,
}
