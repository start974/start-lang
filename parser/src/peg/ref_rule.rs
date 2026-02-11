use location::{GetSpan, SetSpan, Span};

#[derive(Debug)]
pub struct RefRule {
    /// name of the rule being referenced, e.g. `expr` in `expr = other` where `other` is the name of the rule being referenced
    pub name: String,

    /// span of the rule reference, used for error reporting
    span: Span,
}

impl From<&str> for RefRule {
    fn from(s: &str) -> Self {
        RefRule {
            name: s.to_string(),
            span: Span::default(),
        }
    }
}

impl GetSpan for RefRule {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for RefRule {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
