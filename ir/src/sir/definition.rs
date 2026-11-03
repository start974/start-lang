use crate::location::{Span, Spanned};
use crate::sir::{Documentation, Node};
use std::fmt;

// ============================================================================
// Definition Identifier
// ============================================================================

/// identifier of a definition (expression or type), given by the lowering
/// and unique in a project: the `n`-th definition of the project gets `n`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefId(pub usize);

impl fmt::Display for DefId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

// ============================================================================
// File Identifier
// ============================================================================

/// identifier of a file (a source) of a project: the `n`-th file lowered
/// gets `n`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub u32);

// ============================================================================
// Definition Information
// ============================================================================

/// what a definition defines
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefKind {
    Expression,
    Type,
}

/// what is known about a definition before typing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefInfo {
    /// defined name and its span
    pub name: Node<String>,
    pub kind: DefKind,
    pub file: FileId,
    /// span of the whole definition (`a : Nat := 3`, `T := Nat`)
    pub span: Span,
    pub doc: Option<Documentation>,
}

impl Spanned for DefInfo {
    fn span(&self) -> Span {
        self.span
    }
}
