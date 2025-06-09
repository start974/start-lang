use crate::utils::theme::Theme;

use ariadne::FileCache;
use std::{collections::VecDeque, io::Write};

type SourceId = String;

pub trait ErrorWriter {
    /// cache of writer
    fn cache(&mut self) -> FileCache;

    /// theme
    fn theme(&self) -> &Theme;

    /// writer
    fn writer(&mut self) -> Box<dyn Write>;
}

pub struct DefaultErrorWriter {
    cache: FileCache,
    theme: Theme,
}

impl DefaultErrorWriter {
    pub fn new(cache: FileCache, theme: Theme) -> Self {
        Self { cache, theme }
    }
}

impl ErrorWriter for DefaultErrorWriter {
    fn cache(&mut self) -> FileCache {
        self.cache.clone()
    }
    fn theme(&self) -> &Theme {
        &self.theme
    }
    fn writer(&mut self) -> Box<dyn std::io::Write> {
        Box::new(std::io::stdout())
    }
}

pub trait Error {
    /// error code
    fn code(&self) -> i32;

    /// write error
    fn write(&self, writer: &mut dyn ErrorWriter);
}

pub struct Errors {
    errors: VecDeque<Box<dyn Error>>,
}

impl Errors {
    /// empty errors
    pub fn empty() -> Self {
        Self {
            errors: VecDeque::new(),
        }
    }

    /// add other error
    pub fn add(mut self, err: impl Error + 'static) -> Self {
        self.errors.push_back(Box::new(err));
        self
    }

    /// contant errors
    pub fn concat(mut self, mut other: Self) -> Self {
        self.errors.append(&mut other.errors);
        self
    }
}

impl Error for Errors {
    fn code(&self) -> i32 {
        if self.errors.len() > 1 {
            1
        } else {
            self.errors.front().map_or(0, |e| e.code())
        }
    }

    fn write(&self, writer: &mut dyn ErrorWriter) {
        for error in &self.errors {
            error.write(writer);
            writer.writer().flush().unwrap();
        }
    }
}
