use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, SourceId};
use chumsky::error::Rich;

pub struct Error<'src> {
    loc: Location,
    err: Rich<'src, char>,
}

impl<'src> Error<'src> {
    /// make a new error
    pub fn new(err: Rich<'src, char>, source_id: SourceId, offset: usize) -> Self {
        let span = err.span();
        let loc = Location::new(source_id, span.start, span.end).with_offset(offset);
        Self {
            loc,
            err: err.clone(),
        }
    }
}

impl ErrorCode for Error<'_> {
    fn code(&self) -> i32 {
        201
    }
}

impl Located for Error<'_> {
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl ErrorReport for Error<'_> {
    fn head(&self) -> Message {
        Message::text("Lexing error")
    }

    fn text(&self) -> Option<Message> {
        let mut msg = Message::nil();
        if self.err.expected().len() == 1 {
            msg.add_text("Lexer expected ");
            let expect_str = self.err.expected().next().unwrap().to_string();
            msg.extend(Message::quoted(expect_str));
            if self.err.found().is_some() {
                msg.add_text(", found ")
            }
        } else {
            msg.add_text("Lexer unknow token ");
        };
        if let Some(found) = self.err.found() {
            msg.extend(Message::quoted(found.to_string().escape_default()).important());
        }
        msg.add_text(".");
        Some(msg)
    }
}
