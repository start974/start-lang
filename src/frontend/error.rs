use core::fmt;
use std::io;

use super::lexer::{utils::TokenResult, token::Token};

#[derive(Debug)]
pub enum ErrorKind {
    EOF,
    EOL,
    Whitespaces,
    Tag(String),
    Context(String),
}

#[derive(Debug)]
pub struct ParsingErr {
    token: Token,
    kind: ErrorKind,
}

impl ParsingErr {
    pub fn new(kind: ErrorKind, token: Token) -> Self {
        ParsingErr { token, kind }
    }

    pub fn result<T>(kind: ErrorKind, token: Token) -> TokenResult<T> {
        Err(Self::new(kind, token))
    }

    pub fn token(&self) -> Token {
        self.token.clone()
    }
}

pub enum Error {
    Io(io::Error),
    Parsing(ParsingErr),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => f.debug_tuple("Io").field(err).finish(),
            Self::Parsing(err) => f.debug_tuple("Parsing").field(err).finish(),
        }
    }
}
