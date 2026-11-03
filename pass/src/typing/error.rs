//! Diagnostics of the typing errors (codes 3xx; 301, an unknown name, is
//! found by the lowering).

use errors::{Error, Message};
use ir::location::Span;
use ir::tir;

/// the type `found` was restricted to the incompatible type `expected`,
/// written at `span`
pub fn unexpected_type(expected: &tir::Type, found: &tir::Type, span: Span) -> Error {
    let found = Message::of_pretty(found).important();
    Error::new(302, Message::text("Type mismatch."))
        .with_span(span)
        .with_text(
            Message::text("Found type ")
                .append(found.clone())
                .with_text("."),
        )
        .with_note(
            Message::text("Expected : ")
                .append(Message::of_pretty(expected).important())
                .with_line()
                .with_text("Found    : ")
                .append(found.clone()),
        )
        .with_help(
            Message::text("change this expression so that its type is ")
                .append(Message::of_pretty(expected).important())
                .with_text(", not ")
                .append(found),
        )
}
