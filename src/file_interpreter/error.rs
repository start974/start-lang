use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location};
use std::path::PathBuf;

//=======================================================================
//File Read Error
//=======================================================================

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
        101
    }
}

impl Located for ErrorFileRead {
    fn loc(&self) -> Location {
        Location::unknown()
    }
}

impl ErrorReport for ErrorFileRead {
    fn head(&self) -> Message {
        Message::nil()
            .text("Cannot read file ")
            .quoted(self.path.to_string_lossy())
            .text(".")
    }
}
