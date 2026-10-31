use crate::{Documentation, Variable, ty};
use location::{Span, Spanned};

// ============================================================================
// Type Definition
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition {
    pub name: Variable,
    pub ty: ty::Type,
    pub doc: Option<Documentation>,
}

impl Spanned for TypeDefinition {
    fn span(&self) -> Span {
        self.name.span().union(self.ty.span())
    }
}
