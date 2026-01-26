use crate::Identifier;
use location::{Span, GetSpan, SetSpan};
use pp::pretty::*;
use std::rc::Rc;

// ==========================================================================
// Pattern Variable
// ==========================================================================
pub struct PatternVar {
    /// identifier of the variable
    id: Rc<Identifier>,
    /// span of the pattern
    span: Span,
}

impl PatternVar {
    /// get identifier
    pub fn identifier(&self) -> &Identifier {
        &self.id
    }
}

impl From<Rc<Identifier>> for PatternVar {
    fn from(id: Rc<Identifier>) -> Self {
        Self {
            id,
            span: Span::default(),
        }
    }
}

impl Pretty for PatternVar {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.def_var(&self.id)
    }
}

impl GetSpan for PatternVar {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for PatternVar {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

// ==========================================================================
// Pattern
// ==========================================================================
pub enum Pattern {
    Variable(PatternVar),
}

impl Pattern {
    /// get names on patterns
    pub fn names(&self) -> impl Iterator<Item = &Identifier> {
        match self {
            Pattern::Variable(var) => std::iter::once(var.id.as_ref()),
        }
    }
}

impl Pretty for Pattern {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Pattern::Variable(var) => var.pretty(theme),
        }
    }
}

impl GetSpan for Pattern {
    fn span(&self) -> Span {
        match self {
            Pattern::Variable(var) => var.span(),
        }
    }
}
