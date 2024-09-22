use crate::location;
use crate::location::Location;

pub use colored::{ColoredString, Colorize};
use std::collections::VecDeque;

pub trait Message {
    /// make new message
    fn new() -> Self
    where
        Self: Sized;

    /// push a colored string
    fn push(self, msg: ColoredString) -> Self
    where
        Self: Sized;

    /// colored text
    fn text_colored(&self, msg: &str) -> ColoredString;

    /// colored important
    fn important_colored(&self, msg: &str) -> ColoredString;

    /// iter over colored strings
    fn iter(&self) -> std::slice::Iter<ColoredString>;

    /// len of message
    fn len(&self) -> usize;

    /// is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// make text message with color style without space
    fn text_(self, msg: &str) -> Self
    where
        Self: Sized,
    {
        let msg_colored = self.text_colored(msg);
        self.push(msg_colored)
    }

    /// add space
    fn space(self) -> Self
    where
        Self: Sized,
    {
        self.text_(" ")
    }

    /// make text with space if necessary
    fn text(self, msg: &str) -> Self
    where
        Self: Sized,
    {
        if self.is_empty() {
            self.text_(msg)
        } else {
            self.space().text_(msg)
        }
    }

    /// add important message
    fn important_(self, msg: &str) -> Self
    where
        Self: Sized,
    {
        let msg_colored = self.important_colored(msg);
        self.push(msg_colored)
    }

    /// add important message preceded by space
    fn important(self, msg: &str) -> Self
    where
        Self: Sized,
    {
        if self.is_empty() {
            self.important_(msg)
        } else {
            self.space().important_(msg)
        }
    }

    /// add quoted message
    fn quoted_(self, msg: &str) -> Self
    where
        Self: Sized,
    {
        self.text_("\"").text_(msg).text_("\"")
    }

    /// add quoted message preceded by space
    fn quoted(self, msg: &str) -> Self
    where
        Self: Sized,
    {
        if self.is_empty() {
            self.quoted_(msg)
        } else {
            self.space().quoted_(msg)
        }
    }
}

impl std::fmt::Display for dyn Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for msg in self.iter() {
            write!(f, "{}", msg)?;
        }
        if !self.is_empty() {
            writeln!(f, "{}", self.text_colored("."))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Head(Vec<ColoredString>);
impl Message for Head {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push(mut self, msg: ColoredString) -> Self {
        self.0.push(msg);
        self
    }
    fn text_colored(&self, msg: &str) -> ColoredString {
        msg.red()
    }
    fn important_colored(&self, msg: &str) -> ColoredString {
        msg.red().bold()
    }
    fn iter(&self) -> std::slice::Iter<ColoredString> {
        self.0.iter()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
pub struct Hint(Vec<ColoredString>);
impl Message for Hint {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push(mut self, msg: ColoredString) -> Self {
        self.0.push(msg);
        self
    }
    fn text_colored(&self, msg: &str) -> ColoredString {
        msg.into()
    }
    fn important_colored(&self, msg: &str) -> ColoredString {
        msg.yellow().bold()
    }
    fn iter(&self) -> std::slice::Iter<ColoredString> {
        self.0.iter()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
pub struct Error {
    code: i32,
    head: Head,
    hints: Vec<Hint>,
    location: Option<Location>,
}

impl Error {
    /// make simple error
    pub fn make(head: Head, code: i32) -> Self {
        Self {
            code,
            head,
            hints: Vec::new(),
            location: None,
        }
    }

    /// add hint to error
    pub fn add_hint(mut self, hint: Hint) -> Self {
        self.hints.push(hint);
        self
    }

    /// get code
    pub fn get_code(&self) -> i32 {
        self.code
    }
}

impl location::Located for Error {
    fn get_location(&self) -> &Option<Location> {
        &self.location
    }

    fn set_opt_location(mut self, opt_location: Option<Location>) -> Self {
        self.location = opt_location;
        self
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &format!("Error[{}]: ", self.code).red().bold())?;
        if let Some(location) = &self.location {
            location.fmt_location(f)?;
            writeln!(f)?;
            location.fmt_content(f)?;
            location.fmt_indicator(f)?;
            write!(f, " ")?;
        }
        write!(f, "{}", &self.head as &dyn Message)?;
        if !self.hints.is_empty() {
            writeln!(f, "\n{}", "Hints:".yellow().bold())?;
            let n = self.hints.len();
            for (i, hint) in self.hints.iter().enumerate() {
                write!(f, "{}", hint as &dyn Message)?;
                if i < n - 1 {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Errors {
    errors: VecDeque<Error>,
}

impl Errors {
    /// empty errors
    pub fn empty() -> Self {
        Self {
            errors: VecDeque::new(),
        }
    }

    /// add other error
    pub fn add_error(mut self, err: Error) -> Self {
        self.errors.push_back(err);
        self
    }

    /// get error code
    pub fn get_code(&self) -> i32 {
        if self.errors.len() == 1 {
            self.errors[0].get_code()
        } else {
            1
        }
    }
}

impl From<Error> for Errors {
    fn from(error: Error) -> Self {
        Self::empty().add_error(error)
    }
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.errors.len();
        for (i, err) in self.errors.iter().enumerate() {
            write!(f, "{}", err)?;
            if i < n - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
