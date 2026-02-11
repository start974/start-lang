use errors::{Error, Message};
use location::Span;

// ===========================================================================
// Error rule construction
// ===========================================================================
pub fn not_defined(name: &str, span: Span) -> Error {
    Error::new(201, Message::text("Gramar rule not defined."))
        .with_text(
            Message::text("Grammar rule ")
                .with_quoted(name)
                .with_text("does not exist."),
        )
        .with_span(span)
}

// ===========================================================================
// Parser Error
// ===========================================================================
/*#[derive(Debug)]*/
/*pub struct Expected {*/
/*/// The syntax that was expected at this point.*/
/*pub expected: String,*/

/*/// position*/
/*pub position: usize,*/
/*}*/

/*#[derive(Debug, Default)]*/
/*pub struct ParserError {*/
/*/// expected syntax at the point of failure, used for error reporting*/
/*expected: Vec<Expected>,*/
/*}*/
