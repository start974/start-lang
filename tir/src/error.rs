use crate::env::IdentifierKind;
use crate::{Identifier, Type};
use errors::{Error, Message};
use location::Span;

pub fn variable_not_found(identifier: Identifier, kind: IdentifierKind, span: Span) -> Error {
    Error::new(301, Message::text("Variable not found."))
        .with_span(span)
        .with_text({
            match kind {
                IdentifierKind::Type => Message::text("Type variable "),
                IdentifierKind::Expr => Message::text("Expression variable "),
                IdentifierKind::Unknown => Message::text("Variable "),
            }
            .append(Message::text(identifier.name()).important())
            .with_text(" not found.")
        })
}

pub fn unexpected_type(expected: &Type, found: &Type, span: Span) -> Error {
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
                .append(found),
        )
}
