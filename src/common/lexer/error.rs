use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, SourceId};
use chumsky::error::Rich;

pub struct Error {
    loc: Location,
    expected: Option<String>,
    found: Option<char>,
}

impl Error {
    /// make a new error
    pub fn new(err: Rich<'_, char>, source_id: SourceId, offset: usize) -> Self {
        let span = err.span();
        Self {
            loc: Location::new(source_id, span.start, span.end).with_offset(offset),
            expected: err.expected().next().map(ToString::to_string),
            found: err.found().cloned(),
        }
    }
}

impl ErrorCode for Error {
    fn code(&self) -> i32 {
        201
    }
}

impl Located for Error {
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl ErrorReport for Error {
    fn head(&self) -> Message {
        Message::text("Lexing error")
    }

    fn text(&self) -> Option<Message> {
        let mut msg = Message::nil();
        match &self.expected {
            Some(expected) => {
                msg.add_text("Lexer expected ");
                msg.extend(Message::quoted(expected));
                if self.found.is_some() {
                    msg.add_text(", found ")
                }
            }
            None => msg.add_text("Lexer unknown token "),
        };

        if let Some(found) = self.found {
            msg.extend(Message::quoted(found.to_string().escape_default()).important());
        }
        msg.add_text(".");
        Some(msg)
    }
}
