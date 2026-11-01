//! Diagnostics of the vm errors (codes 9xx): internal errors, `codegen` made
//! code that cannot run, so this is a bug of the compiler.

use errors::{Error, Message};

/// read of a global never computed, in `context`
pub fn undefined(context: &str) -> Error {
    Error::new(902, Message::text("Internal error."))
        .with_text(Message::text(format!("vm: undefined global in {context}.")))
}
