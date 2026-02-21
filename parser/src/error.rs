use crate::peg::Peg;
use errors::{Error, IntoError, Message};
use std::collections::HashSet;

// ===========================================================================
// Parser Error
// ===========================================================================
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    /// Farthest failure position in the input
    position: usize,

    /// All syntaxes that were expected at that exact position
    expected: HashSet<Peg>,
}

impl ParseError {
    /// Create a new ParseError with the given position, expected syntax, and found character.
    pub fn new(position: usize, expected: Peg) -> Self {
        Self {
            position,
            expected: HashSet::from([expected]),
        }
    }
    /// union this error with another error, keeping the one with the farthest position.
    pub fn union(mut self, other: Self) -> Self {
        if other.position > self.position {
            other
        } else if other.position == self.position {
            self.expected.extend(other.expected);
            self
        } else {
            self
        }
    }

    /// position of the error in the input
    pub fn position(&self) -> usize {
        self.position
    }
}

impl IntoError for ParseError {
    fn into_error(self) -> Error {
        use location::Span;
        Error::new(100, Message::text("Syntax error."))
            .with_text(
                Message::text("Expected one of:".to_string())
                    .with_line()
                    .append(Message::intersperse(
                        self.expected
                            .into_iter()
                            .map(|peg| Message::text("- ").with_pretty(&peg)),
                        Message::line(),
                    )),
            )
            .with_span(Span::from(self.position))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_parse() {
        let error = ParseError::new(5, Peg::Literal("a".into()))
            .union(ParseError::new(5, Peg::Literal("b".into())))
            .union(ParseError::new(4, Peg::Literal("c".into())));
        assert_eq!(error.position, 5);
        assert_eq!(error.expected.len(), 2);
        assert!(error.expected.contains(&Peg::Literal("a".into())));
        assert!(error.expected.contains(&Peg::Literal("b".into())));
        assert!(!error.expected.contains(&Peg::Literal("c".into())));
    }
}
