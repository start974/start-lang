use crate::frontend::error::ParsingErr;
use crate::frontend::lexer::Lexer;

pub type TokenResult<O> = Result<O, ParsingErr>;
pub trait Parse<'l> {
    fn parse(lexer: &'l mut Lexer<'_>) -> TokenResult<Box<Self>>;
}

pub trait ParseContext<'l, 'c, C> {
    fn parse(lexer: &'l mut Lexer<'_>, context: &'c C) -> TokenResult<Box<Self>>;
}
