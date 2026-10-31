use crate::common::BuiltinType;
use location::{Span, Spanned, SpannedSet};
use pp::pretty::*;

// ==========================================================================
// Builtin Ty
// ==========================================================================
#[derive(Debug, Clone)]
pub struct Builtin {
    /// which builtin type
    kind: BuiltinType,
    /// span of builtin type
    span: Span,
}

impl Builtin {
    /// Create a new builtin type
    fn new(kind: BuiltinType) -> Self {
        Builtin {
            kind,
            span: Span::default(),
        }
    }

    /// make a nat builtin type
    pub fn nat() -> Self {
        Builtin::new(BuiltinType::Nat)
    }

    /// make a char builtin type
    pub fn char() -> Self {
        Builtin::new(BuiltinType::Char)
    }

    /// make a bool builtin type
    pub fn bool() -> Self {
        Builtin::new(BuiltinType::Bool)
    }

    /// get which builtin type
    pub fn kind(&self) -> BuiltinType {
        self.kind
    }
}

impl From<BuiltinType> for Builtin {
    fn from(kind: BuiltinType) -> Self {
        Builtin::new(kind)
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for Builtin {}

impl Pretty for Builtin {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.ty_var(&self.kind.symbol())
    }
}

impl Spanned for Builtin {
    fn span(&self) -> Span {
        self.span
    }
}

impl SpannedSet for Builtin {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
