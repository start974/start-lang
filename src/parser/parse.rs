use super::super::ast::{
    untyped::{Definition, Program},
    Env, Ident,
};
use super::super::location::{Location, Position};
use super::error::{Errors, ErrorsResult};
use super::iter_node::IterNode;

use tree_sitter::Node;

pub struct Parser {
    file_name: String,
    content: Vec<String>,
    env: Env,
}

fn repeat<F, T>(node: &Node, mut result: T, mut f: F) -> T
where
    F: FnMut(T, &Node, usize) -> T,
{
    let mut cursor = node.walk();
    let mut i = 0;
    if cursor.goto_first_child() {
        loop {
            result = f(result, &cursor.node(), i);
            i += 1;
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    result
}

type ParserResult<T> = (Parser, ErrorsResult<T>);

impl Parser {
    pub fn make(file_name: &String, content: &[String]) -> Self {
        Self {
            file_name: file_name.to_owned(),
            content: content.to_vec(),
            env: Env::empty(),
        }
    }

    fn location(&self, node: &Node) -> Location {
        let start = node.start_position();
        let end = node.end_position();
        Location {
            file_name: self.file_name.clone(),
            pos_start: Position {
                row: start.row,
                column: start.column,
            },
            pos_end: Position {
                row: end.row,
                column: end.column,
            },
        }
    }

    fn self_error<T>(self, node: &Node, expect: &str) -> ParserResult<T> {
        let error = Errors::error(expect, &self.content, &self.location(node));
        (self, Err(error))
    }

    fn check_keyword(self, node: &Node, expect: &str) -> ParserResult<()> {
        if node.kind() != expect {
            let expect = format!("'{}' keyword", expect);
            self.self_error(node, &expect)
        } else {
            (self, Ok(()))
        }
    }


    fn parse_ident(self, node: &Node) -> ParserResult<Ident> {
        todo!()
        /*        self.check_kind(node, "ident")?;*/
        /*let location = self.location(node);*/
        /*let name_content = location.content(&self.content);*/
        /*let name = if name_content.len() == 1 {*/
        /*name_content[0].clone()*/
        /*} else {*/
        /*panic!("Name content has more than one line.")*/
        /*};*/
        /*let (env, ident) = self.env.make_ident(&name, &Some(location));*/
        /*self.env = env;*/
        /*(self, Ok(ident))*/
    }

    fn parse_expr_def(self, node: &Node) -> ParserResult<Definition> {
        let loc = self.location(node);
        IterNode::new(node, self, ())
            .next(
                &mut |parser, node_def| {
                    parser.check_keyword(node_def, "def")
                },
                &mut |(), ()| (),
            )
            .next(
                &mut |parser, node_ident| {
                    parser.parse_ident(node_ident)
                },
                &mut |(), ident| ident,
            )
            .map_result(|ident| Definition::make_expr_def(ident, Some(loc)))
            .acc_result()
    }

    fn parse_definition(self, node: &Node) -> ParserResult<Definition> {
        match node.kind() {
            "expr_def" => self.parse_expr_def(node),
            _ => self.self_error(node, "definition"),
        }
    }

    /// parse program
    pub fn parse_program(self, node: &Node) -> ParserResult<Program> {
        match node.kind() {
            "program" => IterNode::new(node, self, Program::empty())
                .repeat(
                    &mut |parser, node_def| parser.parse_definition(node_def),
                    &mut |program1, definition| {
                        let (program2, old_def) = program1.add_definition(definition);
                        assert!(old_def.is_none());
                        program2
                    },
                )
                .acc_result(),
            _ => self.self_error(node, "program"),
        }
    }
}
