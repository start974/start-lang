use crate::tir::Command;

// ============================================================================
// File
// ============================================================================

/// typed commands of a file, in source order
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct File {
    pub commands: Vec<Command>,
}
