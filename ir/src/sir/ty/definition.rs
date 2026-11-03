use crate::location::{Span, Spanned};
use crate::sir::{DefId, Node, ty};

// ============================================================================
// Type Definition
// ============================================================================

/// `name := ty`, the definition `id`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
    pub id: DefId,
    pub name: Node<String>,
    pub ty: ty::Type,
}

impl Spanned for TypeDefinition {
    fn span(&self) -> Span {
        self.name.span().union(self.ty.span())
    }
}
