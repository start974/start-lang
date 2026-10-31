use errors::{Error, Message};
use std::path::Path;

pub fn read_file(path: &Path) -> Error {
    Error::new(
        101,
        Message::text("Cannot read file ")
            .append(Message::quoted(path.to_string_lossy()).important())
            .with_text("."),
    )
}

pub fn write_file(path: &Path) -> Error {
    Error::new(
        102,
        Message::text("Cannot write file ")
            .append(Message::quoted(path.to_string_lossy()).important())
            .with_text("."),
    )
}
