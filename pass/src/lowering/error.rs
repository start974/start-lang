//! Diagnostics of the lowering errors: names that resolve to no definition.

use errors::{Error, Message};
use ir::location::Span;
use ir::sir::DefKind;

/// no definition of `kind` (any kind for `None`) named `name` is visible;
/// `hidden_by`: the kind of the definition of that name that hides it
pub fn variable_not_found(
    name: &str,
    kind: Option<DefKind>,
    hidden_by: Option<DefKind>,
    span: Span,
) -> Error {
    Error::new(301, Message::text("Variable not found."))
        .with_span(span)
        .with_text(
            match kind {
                Some(DefKind::Type) => Message::text("Type variable "),
                Some(DefKind::Expression) => Message::text("Expression variable "),
                None => Message::text("Variable "),
            }
            .append(Message::text(name).important())
            .with_text(match hidden_by {
                Some(DefKind::Expression) => {
                    " not found (hidden by an expression of the same name)."
                }
                Some(DefKind::Type) => " not found (hidden by a type of the same name).",
                None => " not found.",
            }),
        )
}
