pub use crate::sir::Identifier;
use crate::tir::{DefKind, Documentation, Type};

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
