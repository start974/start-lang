use crate::location::{Span, Spanned};
use crate::sir::{DefId, Variable, ty};

// ============================================================================
// Type Definition
// ============================================================================

/// `name := ty`; its kind and documentation are in the `DefTable` of the
/// file, under `id`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
    pub id: DefId,
    pub name: Variable,
    pub ty: ty::Type,
}

impl Spanned for TypeDefinition {
    fn span(&self) -> Span {
        self.name.span().union(self.ty.span())
    }
}
