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
