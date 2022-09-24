mod env;
mod node;

use env::Env;

use self::node::Node;

use super::lexer::{utils::TokenResult, Lexer};

pub struct Parser<'a, 'l>(Env<'a, 'l>);

impl<'a, 'l> Parser<'a, 'l> {
    pub fn new(lexer: &'a mut Lexer<'l>) -> Self {
        Parser(Env::new(lexer))
    }

    pub fn parse(&mut self) -> TokenResult<Vec<Node>> {
        self.0.parse()
    }
}
