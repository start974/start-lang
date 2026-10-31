//! internal errors: a phase failed on a program accepted by the previous
//! ones, so this is a bug of the compiler, reported instead of a panic

use errors::{Error, Message};
use ir::location::Span;

pub fn vm(span: Span) -> Error {
    Error::new(902, Message::text("Internal error."))
        .with_span(span)
        .with_text(Message::text("vm: undefined global."))
}
