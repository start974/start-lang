use std::path::PathBuf;

use crate::utils::error::{Error, ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location, Report, ReportBuilder, UNKNOWN_LOCATION};

// =======================================================================
// File Read Error
// =======================================================================

pub struct ErrorFileRead {
    path: PathBuf,
}

impl ErrorFileRead {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl ErrorCode for ErrorFileRead {
    fn code(&self) -> i32 {
        102
    }
}
impl Located for ErrorFileRead {
    fn loc(&self) -> &Location {
        &UNKNOWN_LOCATION
    }
}
impl ErrorReport for ErrorFileRead {
    fn finalize<'a>(
        &self,
        _: &crate::utils::theme::Theme,
        report: ReportBuilder<'a>,
    ) -> Report<'a> {
        report.finish()
    }
    fn message(&self) -> Message {
        Message::nil()
            .text("Cannot read file ")
            .quoted(&self.path.to_string_lossy())
            .text(".")
    }
}

impl Error for ErrorFileRead {}
