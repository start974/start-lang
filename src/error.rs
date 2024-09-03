use super::location::Location;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Error {
    Simple(String),
    Located { location: Location, msg: String },
    Errors(VecDeque<Error>),
}

impl Error {
    /// make simple error
    pub fn error_simple(msg: &str) -> Self {
        Self::Simple(msg.to_string())
    }

    /// make located error
    pub fn error_located(msg: &str, location: Location) -> Self {
        Self::Located {
            location,
            msg: msg.to_string(),
        }
    }

    /// add other error
    pub fn error_add(self, other: Self) -> Self {
        let errors = match (self, other) {
            (Self::Errors(mut errors1), Self::Errors(errors2)) => {
                errors1.extend(errors2);
                errors1
            }
            (Self::Errors(mut errors1), err) => {
                errors1.push_back(err);
                errors1
            }
            (err, Self::Errors(mut errors2)) => {
                errors2.push_front(err);
                errors2
            }
            (err1, err2) => {
                let mut errors = VecDeque::new();
                errors.push_back(err1);
                errors.push_back(err2);
                errors
            }
        };
        Self::Errors(errors)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(msg) => writeln!(f, "Error: {}", msg),
            Self::Located { location, msg } => writeln!(f, "{}Error: {}.", location, msg),
            Self::Errors(errors) => {
                for error in errors {
                    writeln!(f, "{error}")?;
                }
                Ok(())
            }
        }
    }
}
