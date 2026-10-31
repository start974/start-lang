//! Diagnostics of the typing errors (codes 3xx).

use errors::{Error, Message};
use ir::location::Spanned as _;
use ir::tir;
use ir::tir::env::IdentifierKind;

/// option of `Set` / `Unset` that does not exist
pub fn unknown_option(option: &ir::sir::Variable) -> Error {
    Error::new(103, Message::text("Option unknown."))
        .with_span(option.span())
        .with_text(
            Message::text("Option ")
                .append(Message::quoted(option.name()).important())
                .with_text(" is unknown."),
        )
}

/// diagnostic of an error found while building `tir`
pub fn diagnostic(err: tir::error::Error) -> Error {
    let span = err.span();
    match err {
        tir::error::Error::VariableNotFound {
            identifier, kind, ..
        } => variable_not_found(&identifier, kind),
        tir::error::Error::UnexpectedType {
            expected, found, ..
        } => unexpected_type(&expected, &found),
    }
    .with_span(span)
}

fn variable_not_found(identifier: &tir::Identifier, kind: IdentifierKind) -> Error {
    Error::new(301, Message::text("Variable not found.")).with_text(
        match kind {
            IdentifierKind::Type => Message::text("Type variable "),
            IdentifierKind::Expr => Message::text("Expression variable "),
            IdentifierKind::Unknown => Message::text("Variable "),
        }
        .append(Message::text(identifier.name()).important())
        .with_text(" not found."),
    )
}

fn unexpected_type(expected: &tir::Type, found: &tir::Type) -> Error {
    let found = Message::of_pretty(found).important();
    Error::new(302, Message::text("Type mismatch."))
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
