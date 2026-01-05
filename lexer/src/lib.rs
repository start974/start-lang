pub mod error;
pub mod lexing;
pub mod token;

use chumsky::Parser as _;
pub use error::Error;
pub use location::SourceId;
pub use token::MetaToken;

pub use lexing::lexer;

/// apply lexer on [source_id] with [offset] on [content]
pub fn lex<'src>(
    source_id: SourceId,
    offset: usize,
    content: &'src str,
) -> Result<Vec<MetaToken>, Vec<Error<'src>>> {
    lexer(source_id.clone(), offset)
        .parse(content)
        .into_result()
        .map_err(|errs| {
            errs.iter()
                .map(|e| Error::new(e.clone(), source_id.clone(), offset))
                .collect()
        })
}
