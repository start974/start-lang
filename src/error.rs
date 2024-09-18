use crate::location;
use crate::location::Location;
use crate::utils::colored::*;

use std::collections::VecDeque;

type Hint = String;

#[derive(Debug)]
pub struct Error {
    code: i32,
    msg: String,
    hints: Vec<Hint>,
    location: Option<Location>,
}

impl Error {
    /// make simple error
    pub fn make(msg: &str, code: i32) -> Self {
        Self {
            code,
            msg: msg.to_string(),
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
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        /*match self {*/
        /*Self::Simple(err)  => {*/
        /*writeln!(f, "Error[{}]: {}.", err.code, err.msg),*/
        /*}*/
        /*Self::Located {*/
        /*code,*/
        /*location,*/
        /*msg,*/
        /*} => {*/
        /*writeln!(f, "Error[{code}]: {}", location.string_location())?;*/
        /*write!(f, "{}", location.string_content())?;*/
        /*writeln!(f, "{} {}", location.string_indicator(), msg)*/
        /*}*/

        /*Self::Errors(errors) => {*/
        /*for (i, error) in errors.iter().enumerate() {*/
        /*if i != 0 {*/
        /*writeln!(f)?;*/
        /*}*/
        /*write!(f, "{error}")?;*/
        /*}*/
        /*Ok(())*/
        /*}*/
        /*}*/
    }
}
impl Colored for Error {
    /// colored error
    fn colored(&self) -> String {
        todo!()
        /*match self {*/
        /*Self::Simple(err) => {*/
        /*let s = String::new();*/
        /*s += & cformat!("<red><bold>Error[{}]:</bold> {}</red>.\n{}", err.code, err.msg)*/
        /*if !err.hint.is_empty() {*/
        /*cformat!("<bold>Hint:</bold>\n");*/
        /*}*/

        /*for hint in &err.hint {*/
        /*s += &cformat!("{hint}");*/
        /*}*/
        /*}*/
        /*Self::Located {*/
        /*code,*/
        /*location,*/
        /*msg,*/
        /*} => {*/
        /*let mut res = String::new();*/
        /*res += &cformat!(*/
        /*"<red><bold>Error[{code}]</></> {}\n",*/
        /*location.string_location()*/
        /*);*/
        /*res += &cformat!("{}", location.colored_content());*/
        /*res += &cformat!("<red><bold>{}</> {}</>\n", location.string_indicator(), msg);*/
        /*res*/
        /*}*/
        /*Self::Errors(errors) => {*/
        /*let mut msg = String::new();*/
        /*for (i, error) in errors.iter().enumerate() {*/
        /*if i != 0 {*/
        /*msg += "\n";*/
        /*}*/
        /*msg += &error.colored();*/
        /*}*/
        /*msg*/
        /*}*/
        /*}*/
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
        for err in self.errors.iter() {
            writeln!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl Colored for Errors {
    fn colored(&self) -> String {
        todo!()
    }
}
