use ariadne::Label;

use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder};
use crate::utils::theme::Theme;

// =======================================================================
// Error Kind
// =======================================================================
pub struct ErrorKind {
    expect: String,
    found: String,
    location: Location,
}

impl ErrorKind {
    pub fn new(expect: &str, found: &str, location: Location) -> Self {
        Self {
            expect: expect.to_string(),
            found: found.to_string(),
            location,
        }
    }
}

impl ErrorCode for ErrorKind {
    fn code(&self) -> i32 {
        201
    }
}

impl Located for ErrorKind {
    fn loc(&self) -> &Location {
        &self.location
    }
}

impl ErrorReport for ErrorKind {
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        report
            .with_label(
                Label::new(self.loc().clone()).with_message(
                    Message::nil()
                        .text("Found")
                        .important(&self.found)
                        .text(", expected ")
                        .important(&self.expect)
                        .text(".")
                        .to_string(theme),
                ),
            )
            .finish()
    }
    fn message(&self) -> Message {
        Message::nil()
            .text("Expect ")
            .important(&self.expect)
            .text(".")
    }
}

// =======================================================================
// Error Keyword
// =======================================================================

pub struct ErrorKeyword {
    keyword: String,
    location: Location,
}

impl ErrorKeyword {
    pub fn new(keyword: &str, location: Location) -> Self {
        Self {
            keyword: keyword.to_string(),
            location,
        }
    }
}

impl ErrorCode for ErrorKeyword {
    fn code(&self) -> i32 {
        202
    }
}

impl Located for ErrorKeyword {
    fn loc(&self) -> &Location {
        &self.location
    }
}

impl ErrorReport for ErrorKeyword {
    fn finalize<'a>(&self, _: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        report.finish()
    }

    fn message(&self) -> Message {
        Message::nil()
            .text("Unexpected keyword: ")
            .important(&self.keyword)
            .text(".")
    }
}
