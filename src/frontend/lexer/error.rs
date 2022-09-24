use core::fmt;
use std::{io, str};

use super::{Position};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    UTf8error(str::Utf8Error),
    Unkown(String, Position, String),
}

impl Error {
    pub fn unkown<T>(path: &str, pos: Position, s: &str) -> Result<T, Self> {
        Err(Error::Unkown(path.to_string(), pos, s.to_string()))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(err) => err.fmt(f),
            Error::UTf8error(err) => err.fmt(f),
            Error::Unkown(path, pos, c) => {
                write!(f, "{path}::{}::{}\n unknow\"{c}\"", pos.line(), pos.offset())
            },
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error +'static)>{
        match self {
            Error::IoError(err) => err.source(),
            Error::UTf8error(err) => err.source(),
            Error::Unkown(_, _, _) => None
        }
    }
}