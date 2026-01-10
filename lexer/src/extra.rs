use chumsky::{error::Rich, extra::Err};

pub type ErrorChumsky<'src> = Rich<'src, char>;
pub trait Lexer<'src, T> = chumsky::Parser<'src, &'src str, T, Err<ErrorChumsky<'src>>>;
