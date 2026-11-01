//! Diagnostics of the codegen errors (codes 9xx): internal errors, `typing`
//! accepted a program that cannot be generated, so this is a bug of the
//! compiler.

use errors::{Error, Message};
use ir::location::Spanned as _;
use ir::tir;

/// variable whose definition was never generated
pub fn unknown_identifier(var: &tir::expression::Variable) -> Error {
    Error::new(901, Message::text("Internal error."))
        .with_span(var.span())
        .with_text(Message::text(format!(
            "codegen: unknown identifier `{}`",
            var.identifier()
        )))
}
