use crate::location::Location;
use crate::utils::colored::*;

use std::collections::VecDeque;

#[derive(Debug)]
pub enum Error {
    Simple {
        code: i32,
        msg: String,
    },
    Located {
        code: i32,
        location: Location,
        msg: String,
    },
    Errors(VecDeque<Error>),
}

impl Error {
    /// make simple error
    pub fn error_simple(msg: &str, code: i32) -> Self {
        Self::Simple {
            code,
            msg: msg.to_string(),
        }
    }

    /// make located error
    pub fn error_located(msg: &str, location: Location, code: i32) -> Self {
        Self::Located {
            code,
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

    /// get code
    pub fn get_code(&self) -> i32 {
        match self {
            Self::Simple { code, .. } => *code,
            Self::Located { code, .. } => *code,
            Self::Errors(_) => 1,
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple { code, msg } => writeln!(f, "Error[{code}]: {msg}."),
            Self::Located {
                code,
                location,
                msg,
            } => {
                writeln!(f, "Error[{code}]: {}", location.string_location())?;
                write!(f, "{}", location.string_content())?;
                writeln!(f, "{} {}", location.string_indicator(), msg)
            }

            Self::Errors(errors) => {
                for (i, error) in errors.iter().enumerate() {
                    if i != 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl Colored for Error {
    /// colored error
    fn colored(&self) -> String {
        match self {
            Self::Simple { code, msg } => {
                cformat!("<red><bold>Error[{code}]:</bold> {msg}</red>.\n")
            }
            Self::Located {
                code,
                location,
                msg,
            } => {
                let mut res = String::new();
                res += &cformat!(
                    "<red><bold>Error[{code}]</></> {}\n",
                    location.string_location()
                );
                res += &cformat!("{}", location.colored_content());
                res += &cformat!("<red><bold>{}</> {}</>\n", location.string_indicator(), msg);
                res
            }
            Self::Errors(errors) => {
                let mut msg = String::new();
                for (i, error) in errors.iter().enumerate() {
                    if i != 0 {
                        msg += "\n";
                    }
                    msg += &error.colored();
                }
                msg
            }
        }
    }
}
