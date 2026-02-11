use location::{GetSpan, SetSpan, Span};

// ===========================================================================
// Parser Error
// ===========================================================================
#[derive(Debug)]
pub struct Expected {
    /// The syntax that was expected at this point.
    pub expected: String,

    /// position
    pub position: usize,
}

#[derive(Debug)]
pub struct Found {
    /// The token or character that was found at the point of failure.
    pub found: String,

    /// The span in the input where the found token or character was located.
    pub span: Span,
}

#[derive(Debug, Default)]
pub struct ParserError {
    /// expected syntax at the point of failure, used for error reporting
    expected: Vec<Expected>,
}

// ===========================================================================
// Error Rule
// ===========================================================================
pub enum ErrorRuleKind {
    RuleNotExist(String),
}

pub struct ErrorRule {
    /// The kind of error that occurred.
    kind: ErrorRuleKind,

    /// span of the rule definition, used for error reporting
    span: Span,
}

impl From<ErrorRuleKind> for ErrorRule {
    fn from(kind: ErrorRuleKind) -> Self {
        ErrorRule {
            kind,
            span: Span::default(),
        }
    }
}

impl GetSpan for ErrorRule {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for ErrorRule {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
