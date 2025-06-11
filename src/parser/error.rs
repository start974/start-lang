use ariadne::Label;

use crate::utils::error::{self, ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder};
use crate::utils::theme::Theme;

// =======================================================================
// Error Parser
// =======================================================================
enum Kind {
    Operator,
    Keyword,
    Kind,
}

pub struct Error {
    location: Location,
    expect: String,
    kind: Kind,
}

impl Error {
    pub fn operator(expect: &str, location: Location) -> Self {
        Self {
            location,
            kind: Kind::Operator,
            expect: expect.to_string(),
        }
    }

    pub fn keyword(expect: &str, location: Location) -> Self {
        Self {
            location,
            kind: Kind::Keyword,
            expect: expect.to_string(),
        }
    }

    pub fn kind(expect: &str, location: Location) -> Self {
        Self {
            location,
            kind: Kind::Kind,
            expect: expect.to_string(),
        }
    }
}

impl ErrorCode for Error {
    fn code(&self) -> i32 {
        match self.kind {
            Kind::Operator => 203,
            Kind::Keyword => 202,
            Kind::Kind => 201,
        }
    }
}

impl Located for Error {
    fn loc(&self) -> &Location {
        &self.location
    }
}

impl ErrorReport for Error {
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        report
            .with_label(
                Label::new(self.loc().clone()).with_message(
                    Message::nil()
                        .text("Expect ")
                        .important(&self.expect)
                        .text(".")
                        .to_string(theme),
                ),
            )
            .finish()
    }
    fn message(&self) -> Message {
        Message::nil()
            .text("Unexpected ")
            .important(match self.kind {
                Kind::Operator => "operator",
                Kind::Keyword => "keyword",
                Kind::Kind => "kind",
            })
            .text(".")
    }
}

impl error::Error for Error {}
