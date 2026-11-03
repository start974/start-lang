use crate::location::{Span, Spanned};
use crate::sir::{DefId, Identifier, Node, ty};
use pp::pretty::*;

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

impl Pretty for TypeDefinition {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let name = Identifier {
            name: self.name.value.clone(),
            id: self.id,
        };
        theme
            .ty_var(&name)
            .append(Doc::space())
            .append(theme.operator(&":="))
            .append(
                Doc::softline()
                    .append(self.ty.pretty(theme).group())
                    .nest(2),
            )
    }
}

impl Spanned for TypeDefinition {
    fn span(&self) -> Span {
        self.name.span().union(self.ty.span())
    }
}
