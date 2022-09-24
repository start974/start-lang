use core::fmt;

use crate::frontend::lexer::utils::{Parse, TokenResult};
use crate::frontend::lexer::Lexer;
use crate::frontend::notation::Notation;

pub enum Node {
    Notation(Notation),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Notation(not) => write!(f, "{not}"),
        }
    }
}
impl<'l> Parse<'l> for Node {
    fn parse(lexer: &'l mut Lexer) -> TokenResult<Box<Self>> {
        Notation::parse(lexer).map(|n| Box::new(Node::Notation(*n)))
    }
}
