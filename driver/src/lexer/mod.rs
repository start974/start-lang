use errors::Errors;
pub mod error;
pub mod lexing;

use ir::token::MetaTokenStream;

pub use lexing::lexer;

pub type ErrorChumsky<'src> = chumsky::error::Rich<'src, char>;

/// lex `content`, whose first byte is at `offset` in its source
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
