use crate::lexer::Token;
use ast::Command;

pub mod ast;
pub mod error;
//pub mod parsing;

pub use error::Error;
//pub type ErrorChumsky<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, Token>>;

///// parse with lexer tokens
/*pub fn parser<'tokens, I>() -> impl Parser<'tokens, I, Command, ErrorChumsky<'tokens>>*/
/*where*/
    /*I: ValueInput<'tokens, Token = Token<'src>>,*/
/*{*/
    /*todo!()*/
/*}*/

/// parse tokens
pub fn parse<'tokens>(_tokens: &'tokens [Token]) -> Result<Command, Error<'tokens>> {
    todo!()
}
