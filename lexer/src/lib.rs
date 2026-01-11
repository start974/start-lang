
use errors::Errors;
pub mod error;
pub mod lexing;
pub mod token;

pub use token::MetaToken;
pub use token::MetaTokenStream;

pub use lexing::lexer;

pub type ErrorChumsky<'src> = chumsky::error::Rich<'src, char>;

/// apply lexer on [source_id] with [offset] on [content]
pub fn lex(content: &str, offset: usize) -> Result<MetaTokenStream, Errors> {
    use chumsky::Parser as _;
    lexer(offset)
        .parse(content)
        .into_result()
        .map(MetaTokenStream::from)
        .map_err(|errs| {
            errs.iter()
                .map(|e| error::error_lexing(e, offset))
                .collect()
        })
}
