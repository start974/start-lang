use super::ast::{Identifier, Type};
use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location};

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
    fn loc(&self) -> Location {
        self.identifier.loc()
    }
}

impl ErrorReport for ErrorVariableNotFound {
    fn head(&self) -> crate::utils::error::Message {
        Message::text("Variable not found.")
    }

    fn text(&self) -> Option<Message> {
        let msg = Message::text("Variable ")
            .append(Message::text(self.identifier.name()).important())
            .with_text(" not found in the current scope.");
        Some(msg)
    }
}

// =======================================================================
// Error Unexpected Type
// =======================================================================
pub struct ErrorUnexpectedType {
    expected: Type,
    found: Type,
    loc: Location,
}

impl ErrorUnexpectedType {
    pub fn new(expected: &Type, found: &Type, location: &Location) -> Self {
        Self {
            expected: expected.clone(),
            found: found.clone(),
            loc: location.clone(),
        }
    }
}

impl ErrorCode for ErrorUnexpectedType {
    fn code(&self) -> i32 {
        302
    }
}

impl Located for ErrorUnexpectedType {
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl ErrorReport for ErrorUnexpectedType {
    fn head(&self) -> crate::utils::error::Message {
        Message::text("Type mismatch.")
    }

    fn text(&self) -> Option<Message> {
        let msg = Message::text("Found type: ")
            .append(Message::of_pretty(&self.found).important())
            .with_text(".");
        Some(msg)
    }

    fn note(&self) -> Option<Message> {
        let msg = Message::
            text("Expected : ")
            .append(Message::of_pretty(&self.expected).important())
            .with_line()
            .with_text("Found    : ")
            .append(Message::of_pretty(&self.found).important());
        Some(msg)
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
    fn loc(&self) -> Location {
        match self {
            Error::VariableNotFound(e) => e.loc(),
            Error::UnexpectedType(e) => e.loc(),
        }
    }
}

impl ErrorReport for Error {
    fn head(&self) -> crate::utils::error::Message {
        match self {
            Error::VariableNotFound(e) => e.head(),
            Error::UnexpectedType(e) => e.head(),
        }
    }

    fn text(&self) -> Option<Message> {
        match self {
            Error::VariableNotFound(e) => e.text(),
            Error::UnexpectedType(e) => e.text(),
        }
    }

    fn note(&self) -> Option<Message> {
        match self {
            Error::VariableNotFound(e) => e.note(),
            Error::UnexpectedType(e) => e.note(),
        }
    }
}
