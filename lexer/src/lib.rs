#![feature(trait_alias)]

use chumsky::Parser as _;
use errors::Errors;

pub mod error;
pub mod lexing;
pub mod token;

pub use token::MetaToken;
pub use token::MetaTokenStream;

pub use lexing::lexer;

pub type ErrorChumsky<'src> = chumsky::error::Rich<'a, char, location::Span>;

/// apply lexer on [source_id] with [offset] on [content]
pub fn lex<I>(it: I) -> Result<MetaTokenStream, Errors> {
    lexer()
        .parse(content)
        .into_result()
        .map(MetaTokenStream::from)
        .map_err(|errs| {
            errs.iter()
                .map(|e| error::error_lexing(e.clone(), offset))
                .collect()
        })
}
