use crate::sir::{Command, DefId, DefInfo, FileId};
use std::collections::BTreeMap;

// ============================================================================
// File
// ============================================================================

/// commands of a file, in source order, with the tables of its definitions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub file: FileId,
    pub commands: Vec<Command>,
    /// the definitions of the file, with their documentation, in source order
    pub defs: BTreeMap<DefId, DefInfo>,
}
