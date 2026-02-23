use location::{GetSpan, SetSpan, Span};
use pp::pretty::*;

#[derive(Debug, Clone)]
pub struct Token {
    /// content of the token
    pub content: String,
    /// span of the token
    span: Span,
}

impl From<&str> for Token {
    fn from(content: &str) -> Self {
        Token {
            content: content.to_string(),
            span: Span::default(),
        }
    }
}

impl GetSpan for Token {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Token {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl Pretty for Token {
    fn pretty(&self, _theme: &Theme) -> Doc<'_> {
        Doc::text(self.content.clone())
    }
}
