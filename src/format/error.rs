use crate::utils::error::{ErrorCode, ErrorReport, Message};
use crate::utils::location::{Located, Location};
use std::path::PathBuf;

//=======================================================================
//File Read Error
//=======================================================================

pub struct ErrorFileWrite {
    path: PathBuf,
}

impl ErrorFileWrite {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl ErrorCode for ErrorFileWrite {
    fn code(&self) -> i32 {
        102
    }
}
impl Located for ErrorFileWrite {
    fn loc(&self) -> Location {
        Location::unknown()
    }
}
impl ErrorReport for ErrorFileWrite {
    fn head(&self) -> Message {
        Message::nil()
            .text("Cannot write file ")
            .quoted(self.path.to_string_lossy())
            .text(".")
    }
}
