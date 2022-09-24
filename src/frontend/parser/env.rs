use crate::frontend::lexer::{
    utils::{Parse, TokenResult},
    Lexer,
};

use super::node::Node;

pub struct Env<'a, 'l> {
    lexer: &'a mut Lexer<'l>,
    // notations: NotationEnv,
    // expressions: ExprEnv,
}

impl<'a, 'l> Env<'a, 'l> {
    pub fn new(lexer: &'a mut Lexer<'l>) -> Self {
        Env {
            lexer,
            // notations: NotationEnv::empty(),
            // expressions: ExprEnv::new(),
        }
    }

    pub fn parse(&mut self) -> TokenResult<Vec<Node>> {
        let node = Node::parse(self.lexer)?;
        Ok(vec![*node])
    }
}
