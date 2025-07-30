use super::ast::{Identifier, Type};
use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder};
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use ariadne::Label;

// =======================================================================
// Error Variable Not Found
// =======================================================================

#[derive(Debug)]
pub struct ErrorVariableNotFound {
    identifier: Identifier,
}

impl ErrorVariableNotFound {
    pub fn new(identifier: Identifier) -> Self {
        Self { identifier }
    }
}

impl ErrorCode for ErrorVariableNotFound {
    fn code(&self) -> i32 {
        301
    }
}

impl Located for ErrorVariableNotFound {
    fn loc(&self) -> &Location {
        self.identifier.loc()
    }
}

impl ErrorReport for ErrorVariableNotFound {
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        report
            .with_label(
                Label::new(self.identifier.loc().clone()).with_message(
                    Message::nil()
                        .text("Variable ")
                        .quoted(self.identifier.name())
                        .text(" not found in the current scope.")
                        .make_string(theme),
                ),
            )
            .finish()
    }

    fn message(&self) -> crate::utils::error::Message {
        Message::nil().text("Variable not found.")
    }
}

// =======================================================================
// Error Unexpected Type
// =======================================================================
pub struct ErrorUnexpectedType {
    expected: Type,
    found: Type,
    location: Location,
}

impl ErrorUnexpectedType {
    pub fn new(expected: &Type, found: &Type, location: &Location) -> Self {
        Self {
            expected: expected.clone(),
            found: found.clone(),
            location: location.clone(),
        }
    }
}

impl ErrorCode for ErrorUnexpectedType {
    fn code(&self) -> i32 {
        302
    }
}

impl Located for ErrorUnexpectedType {
    fn loc(&self) -> &Location {
        &self.location
    }
}

impl ErrorReport for ErrorUnexpectedType {
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        report
            .with_label(
                Label::new(self.location.clone()).with_message(
                    Message::nil()
                        .text("Expect Type ")
                        .of_pretty(&self.expected)
                        .text(".")
                        .make_string(theme),
                ),
            )
            .with_note(
                Message::nil()
                    .text("Expected type: ")
                    .of_pretty(&self.expected)
                    .text(".\n")
                    .text("Found type:    ")
                    .of_pretty(&self.found)
                    .make_string(theme),
            )
            .finish()
    }
    fn message(&self) -> crate::utils::error::Message {
        Message::nil().text("Type mismatch.")
    }
}

// =======================================================================
// ErrorFromParser
// =======================================================================
pub enum Error {
    VariableNotFound(ErrorVariableNotFound),
    UnexpectedType(ErrorUnexpectedType),
}

impl From<ErrorVariableNotFound> for Error {
    fn from(e: ErrorVariableNotFound) -> Self {
        Error::VariableNotFound(e)
    }
}

impl From<ErrorUnexpectedType> for Error {
    fn from(e: ErrorUnexpectedType) -> Self {
        Error::UnexpectedType(e)
    }
}

impl ErrorCode for Error {
    fn code(&self) -> i32 {
        match self {
            Error::VariableNotFound(e) => e.code(),
            Error::UnexpectedType(e) => e.code(),
        }
    }
}

impl Located for Error {
    fn loc(&self) -> &Location {
        match self {
            Error::VariableNotFound(e) => e.loc(),
            Error::UnexpectedType(e) => e.loc(),
        }
    }
}

impl ErrorReport for Error {
    fn finalize<'a>(&self, theme: &Theme, report: ReportBuilder<'a>) -> Report<'a> {
        match self {
            Error::VariableNotFound(e) => e.finalize(theme, report),
            Error::UnexpectedType(e) => e.finalize(theme, report),
        }
    }
    fn message(&self) -> crate::utils::error::Message {
        match self {
            Error::VariableNotFound(e) => e.message(),
            Error::UnexpectedType(e) => e.message(),
        }
    }
}
