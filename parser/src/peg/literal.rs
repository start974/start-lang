use location::{GetSpan, SetSpan, Span};

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: String,
    span: Span,
}

impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        Literal {
            value: s.to_string(),
            span: Span::default(),
        }
    }
}

impl GetSpan for Literal {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Literal {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
