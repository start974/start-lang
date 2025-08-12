use crate::lexer::token::Token;
use crate::lexer::MetaToken;
use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, SourceId};
use chumsky::error::{Rich, RichPattern};

pub struct Error {
    loc: Location,
    expected: Vec<String>,
    found: Option<Token>,
}

impl Error {
    /// make a new error
    pub fn new(err: Rich<'_, MetaToken>, source_id: SourceId) -> Self {
        let span = err.span();
        let loc = Location::new(source_id, span.start, span.end);
        Self {
            loc,
            expected: {
                err.expected()
                    .map(RichPattern::to_string)
                    .filter(|s| !s.is_empty())
                    .collect()
            },
            found: err.found().map(|meta| meta.value.clone()),
        }
    }
}

impl ErrorCode for Error {
    fn code(&self) -> i32 {
        202
    }
}

impl Located for Error {
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl ErrorReport for Error {
    fn head(&self) -> Message {
        Message::text("Parsing error")
    }

    fn text(&self) -> Option<Message> {
        let msg = Message::text("Parsing Expected ")
            .append(Message::intersperse(
                self.expected.iter().map(|s| Message::quoted(s).important()),
                Message::text(" or "),
            ))
            .with_text(".");
        Some(msg)
    }

    fn note(&self) -> Option<Message> {
        match &self.found {
            None => None,
            Some(found) => {
                let expected_list = Message::intersperse(
                    self.expected.iter().map(|s| Message::quoted(s).important()),
                    Message::text(", "),
                );
                let msg = Message::text("Expected : ")
                    .append(expected_list)
                    .with_line()
                    .with_text("Found    : ")
                    .append(Message::quoted(found.to_string()).important())
                    .with_text(".");
                Some(msg)
            }
        }
    }
}
