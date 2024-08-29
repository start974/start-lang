use super::super::ast::{
    untyped::{Definition, Program},
    Env, Ident,
};
use super::super::location::{Location, Position};
use super::error::{Error, Errors};

use tree_sitter::{Node, TreeCursor};

pub struct Parser {
    file_name: String,
    content: Vec<String>,
    env: Env,
}

type ParserResult<T> = Result<T, Errors>;

/*struct IterNode<T, U> {*/
/*cursor: TreeCursor,*/
/*acc : T,*/
/*res : Result<U>,*/
/*}*/

/*impl IterNode<T, U> {*/
/*fn new<>(node: &Node, init : T) -> Self<T, ()> {*/
/*let cursor = node.walk();*/
/*if cursor.goto_first_child() {*/
/*Self {*/
/*cursor,*/
/*acc: init,*/
/*res: Ok(())*/
/*}*/
/*} else {*/
/*panic!("Node has no children");*/
/*}*/
/*}*/
/*}*/

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

fn seq<F, T>(node: &Node, mut result: T, mut f: [F]) -> T
where
    F: FnMut(T, &Node) -> T,
{
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            result = f(result, &cursor.node());
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    result
}

impl Parser {
    pub fn make(file_name: &String, content: &[String]) -> Self {
        Parser {
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

    fn error<T>(&self, node: &Node, expect: &str) -> ParserResult<T> {
        let err = Errors::error(expect, &self.content, &self.location(node));
        Err(err)
    }

    fn self_error<T>(self, node: &Node, expect: &str) -> (Self, ParserResult<T>) {
        let error = self.error(node, expect);
        (self, error)
    }

    fn parse_ident(mut self, node: &Node) -> (Self, ParserResult<Ident>) {
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

    fn parse_expr_def(mut self, node: &Node) -> (Self, ParserResult<Definition>) {
        //self.check_kind(node, "expr_def")?;
        todo!()
    }

    fn parse_definition(mut self, node: &Node) -> (Self, ParserResult<Definition>) {
        match node.kind() {
            "expr_def" => self.parse_expr_def(node),
            _ => self.self_error(node, "definition"),
        }
    }

    /// parse program
    pub fn parse_program(mut self, node: &Node) -> (Self, ParserResult<Program>) {
        match node.kind() {
            "program" => repeat(
                node,
                (self, Ok(Program::empty())),
                |(mut parser, mut res_prog), node, _| {
                    let (parser2, opt_def) = parser.parse_definition(node);
                    let parser = parser2;
                    match (opt_def, res_prog) {
                        (Ok(new_def), Ok(program1)) => {
                            let (program2, old_def) = program1.add_definition(new_def);
                            assert!(old_def.is_none());
                            (parser, Ok(program2))
                        }
                        (Ok(_), Err(errors)) => (parser, Err(errors)),
                        (Err(errors), Ok(_)) => (parser, Err(errors)),
                        (Err(errors1), Err(errors2)) => (parser, Err(errors2.concat(errors1))),
                    }
                },
            ),
            _ => self.self_error(node, "program"),
        }
    }
}
