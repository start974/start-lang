mod cst;
mod error;
mod grammar;

pub use crate::cst::*;
pub use error::*;
pub use grammar::*;

pub trait Scanner {
    /// peek the next character without consuming it
    fn peek(&self) -> Option<char>;

    /// consume and return the next character
    fn next(&mut self) -> Option<char>;

    /// make a checkpoint of the current position in the input (for backtracking)
    fn checkpoint(&self) -> usize;

    /// restore the input to a previous position (backtracking)
    fn rollback(&mut self, pos: usize);
}

pub trait Parser {
    /// parse the input according to the grammar and return an AST or an error
    fn parse(&self, input: &mut impl Scanner) -> Result<Cst, ParseError>;
}

//pub struct Ast;

//pub trait Elaborator {
///// transform the CST into an AST, applying semantic actions and type checking
//fn elaborate(&self, cst: Cst) -> Ast;
//}
