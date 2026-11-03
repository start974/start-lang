use crate::sir::DefId;

// ==========================================================================
// Identifier
// ==========================================================================

/// a definition in `tir`: its name, and the `DefId` given by the lowering
/// (shown `a__3`)
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Identifier {
    pub name: String,
    pub id: DefId,
}

impl Identifier {
    /// get name of Identifier
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", self.name, self.id.0)
    }
}
