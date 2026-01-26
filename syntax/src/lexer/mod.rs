use errors::Errors;
pub mod lexing;
pub mod token;

pub use token::Token;
pub use token::TokenStream;

pub use lexing::lexer;

/// apply lexer on [source_id] with [offset] on [content]
pub fn lex(content: &str, offset: usize) -> Result<TokenStream, Errors> {
    use chumsky::Parser as _;
    lexer(offset)
        .parse(content)
        .into_result()
        .map(TokenStream::from)
        .map_err(|errs| {
            errs.iter()
                .map(|e| crate::error::lexing(e, offset))
                .collect()
        })
}
