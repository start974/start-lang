use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder, SourceId};
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use ariadne::Label;
use chumsky::error::Rich;

pub struct Error<'src> {
    location: Location,
    err: Rich<'src, char>,
}

impl<'src> Error<'src> {
    /// make a new error
    pub fn new(err: Rich<'src, char>, source_id: SourceId, offset: usize) -> Self {
        let span = err.span();
        let location = Location::new(source_id, span.start, span.end).with_offset(offset);
        Self {
            location,
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
    fn loc(&self) -> &Location {
        &self.location
    }
}

impl ErrorReport for Error<'_> {
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        let mut msg = if self.err.expected().len() == 1 {
            Message::nil()
                .text("Lexer expected ")
                .quoted(self.err.expected().next().unwrap().to_string())
        } else {
            Message::nil().text("Lexer unknow token")
        };
        if let Some(found) = self.err.found() {
            msg = msg.text(", found ").quoted(found.to_string());
        }
        msg = msg.text(".");
        report
            .with_label(Label::new(self.loc().clone()).with_message(msg.to_string(theme)))
            .finish()
    }

    fn message(&self) -> Message {
        Message::nil().text("Lexing error")
    }
}
