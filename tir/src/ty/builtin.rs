use location::{Span, GetSpan, SetSpan};
use pp::pretty::*;

// ==========================================================================
// Builtin Kind
// ==========================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// kind of natural numbers
    Nat,
    /// kind of booleans
    Bool,
    /// kind of characters
    Char,
}

impl Pretty for Kind {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Kind::Nat => theme.ty_var(&"ℕ"),
            Kind::Bool => theme.ty_var(&"𝔹"),
            Kind::Char => theme.ty_var(&"Char"),
        }
    }
}

// ==========================================================================
// Builtin Ty
// ==========================================================================
#[derive(Debug, Clone)]
pub struct Builtin {
    /// kind of builtin type
    kind: Kind,
    /// span of builtin type
    span: Span,
}

impl Builtin {
    /// Create a new builtin type
    fn new(kind: Kind) -> Self {
        Builtin {
            kind,
            span: Span::default(),
        }
    }

    /// make a nat builtin type
    pub fn nat() -> Self {
        Builtin::new(Kind::Nat)
    }

    /// make a char builtin type
    pub fn char() -> Self {
        Builtin::new(Kind::Char)
    }

    /// make a bool builtin type
    pub fn bool() -> Self {
        Builtin::new(Kind::Bool)
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
        self.kind.pretty(theme)
    }
}

impl GetSpan for Builtin {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Builtin {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
