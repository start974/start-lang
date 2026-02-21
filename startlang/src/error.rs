use cst::AsIdentifier as _;
use cst::expression::Variable;
use errors::{Error, Message};
use location::GetSpan as _;
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

pub fn unknown_option(option: &Variable) -> Error {
    Error::new(103, Message::text("Option unknown."))
        .with_span(option.span())
        .with_text(
            Message::text("Option ")
                .append(Message::quoted(option.name()).important())
                .with_text(" is unknown."),
        )
}
