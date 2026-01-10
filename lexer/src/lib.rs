use chumsky::Parser as _;
use errors::Errors;

pub mod error;
pub mod lexing;
pub mod token;

pub use location::SourceId;
pub use token::MetaToken;
pub use token::MetaTokenStream;

pub use lexing::lexer;

/// apply lexer on [source_id] with [offset] on [content]
pub fn lex(source_id: SourceId, offset: usize, content: &str) -> Result<MetaTokenStream, Errors> {
    lexer(source_id.clone(), offset)
        .parse(content)
        .into_result()
        .map(MetaTokenStream::from)
        .map_err(|errs| {
            errs.iter()
                .map(|e| error::error_lexing(e.clone(), source_id.clone(), offset))
                .collect()
        })
}
