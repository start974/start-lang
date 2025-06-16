use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder, UNKNOWN_LOCATION};
use crate::utils::theme::Theme;

pub enum Error {}

impl ErrorCode for Error {
    fn code(&self) -> i32 {
        101
    }
}

impl Located for Error {
    fn loc(&self) -> &Location {
        &UNKNOWN_LOCATION
    }
}

impl ErrorReport for Error {
    fn finalize<'a>(&self, _theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        report.finish()
    }

    fn message(&self) -> Message {
        Message::nil().text("Parsing error")
    }
}
