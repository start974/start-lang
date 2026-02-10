use crate::grammar::Syntax;
use location::Span;

#[derive(Debug)]
pub struct Expected {
    /// The syntax that was expected at this point.
    pub syntax: Syntax,

    /// The span in the input where the expected syntax was anticipated.
    pub span: Span,
}

#[derive(Debug)]
pub struct Found {
    /// The token or character that was found at the point of failure.
    pub found: String,

    /// The span in the input where the found token or character was located.
    pub span: Span,
}

#[derive(Debug, Default)]
pub struct ParseError {
    /// expected syntax at the point of failure, used for error reporting
    expected: Vec<Expected>,

    /// found token or character that caused the failure, used for error reporting
    found: Option<Found>,
}

impl ParseError {
    /// Add an expected syntax to the error, with its span in the input.
    pub fn add_expected(mut self, syntax: Syntax, span: Span) -> Self {
        self.expected.push(Expected { syntax, span });
        self
    }

    /// Set the found token or character that caused the failure.
    pub fn found(mut self, found: Found) -> Self {
        self.found = Some(found);
        self
    }
}
