use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder, SourceId};
use crate::utils::theme::Theme;
use ariadne::Label;
use chumsky::error::Rich;

pub struct Error<'src> {
    location: Location,
    err: Rich<'src, char>,
}

impl<'src> Error<'src> {
    /// make a new error
    pub fn new(err: Rich<'src, char>, source_id: SourceId) -> Self {
        let span = err.span();
        let location = Location::new(span.start, span.end, source_id);
        Self {
            location,
            err: err.clone(),
        }
    }
}

impl ErrorCode for Error<'_> {
    fn code(&self) -> i32 {
        101
    }
}

impl Located for Error<'_> {
    fn loc(&self) -> &Location {
        &self.location
    }
}

impl ErrorReport for Error<'_> {
    fn finalize<'a>(&self, _theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        report
            .with_label(Label::new(self.loc().clone()).with_message(self.err.to_string()))
            .finish()
    }

    fn message(&self) -> Message {
        Message::nil().text("Parsing error")
    }
}
