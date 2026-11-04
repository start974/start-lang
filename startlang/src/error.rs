use errors::{Error, Message};
use std::path::Path;

pub fn read_file(path: &Path) -> Error {
    Error::new(
        101,
        Message::text("Cannot read file ")
            .append(Message::quoted(path.to_string_lossy()).important())
            .with_text("."),
    )
    .with_help(Message::text(
        "check that the file exists and that you can read it",
    ))
}

/// `startlang tui` started without a terminal (stdin or stdout redirected)
pub fn no_terminal() -> Error {
    Error::new(103, Message::text("The TUI needs a terminal.")).with_help(Message::text(
        "run `startlang tui` in a terminal, without redirecting its input or output",
    ))
}

pub fn write_file(path: &Path) -> Error {
    Error::new(
        102,
        Message::text("Cannot write file ")
            .append(Message::quoted(path.to_string_lossy()).important())
            .with_text("."),
    )
    .with_help(Message::text(
        "check that the directory exists and that you can write in it",
    ))
}
