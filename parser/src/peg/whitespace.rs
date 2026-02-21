use location::{GetSpan, SetSpan, Span};
use pp::pretty::*;

#[derive(Debug, Clone)]
pub struct WhiteSpace {
    span: Span,
}

impl GetSpan for WhiteSpace {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for WhiteSpace {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl Pretty for WhiteSpace {
    fn pretty(&self, _theme: &Theme) -> Doc<'_> {
        Doc::text("WS")
    }
}

impl PartialEq for WhiteSpace {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for WhiteSpace {}

impl std::hash::Hash for WhiteSpace {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // All WhiteSpace instances are considered equal, so we can hash a constant value
        0.hash(state);
    }
}
