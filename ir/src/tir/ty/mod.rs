use crate::tir::Identifier;
use pp::pretty::*;

// ============================================================================
// Builtin
// ============================================================================

pub use crate::common::BuiltinType;

// ============================================================================
// Type
// ============================================================================

/// a type, without span: the span of a typed node is the span of the node
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Builtin(BuiltinType),
    /// use of a type definition, with the type it stands for
    Alias { name: Identifier, ty: Box<Type> },
}

impl Type {
    /// the builtin type, aliases unfolded
    pub fn builtin(&self) -> BuiltinType {
        match self {
            Type::Builtin(builtin) => *builtin,
            Type::Alias { ty, .. } => ty.builtin(),
        }
    }

    /// same type once aliases are unfolded
    pub fn is_compatible(&self, other: &Self) -> bool {
        self.builtin() == other.builtin()
    }
}

impl Pretty for Type {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Type::Builtin(builtin) => theme.ty_var(&builtin.symbol()),
            Type::Alias { name, .. } => theme.ty_var(&name.name),
        }
    }
}
