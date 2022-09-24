mod token_content;
mod position;
mod simple_lexer;
pub mod token;
mod token_combinator;
pub mod utils;

pub type Lexer<'r> = simple_lexer::Lexer<'r>;
pub type FilePosition = position::FilePosition;
pub type Token = token::Token;
pub type TokenCont = token_content::Content;
