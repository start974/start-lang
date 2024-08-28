use super::super::ast::{
    untyped::{Definition, Program},
    Env, Ident,
};
use super::super::location::{Location, Position};
use super::error::{Error, Errors};

use tree_sitter::Node;

pub struct Parser {
    file_name: String,
    content: Vec<String>,
    env: Env,
}

impl Parser {
    //TODO: add reference to parser with lifetime
    pub fn new(file_name: String, content: Vec<String>) -> Self {
        Parser {
            file_name,
            content,
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

    fn error(&self, node: &Node, expect: &str) -> Error {
        Error::new(expect, &self.content, &self.location(node))
    }

    fn check_kind(&self, node: &Node, expect: &str) -> Result<(), Error> {
        if node.kind() != expect {
            Err(self.error(node, expect))
        } else {
            Ok(())
        }
    }

    fn parse_ident(&mut self, node: &Node) -> Result<Ident, Error> {
        self.check_kind(node, "ident")?;
        let location = self.location(node);
        let name_content = location.content(&self.content);
        let name = if name_content.len() == 1 {
            name_content[0].clone()
        } else {
            panic!("Name content has more than one line.")
        };
        let ident = self.env.make_ident(&name, &Some(location));
        Result::Ok(ident.clone())
    }

    fn parse_expr_def(&mut self, node: &Node) -> Result<Definition, Error> {
        self.check_kind(node, "expr_def")?;
        /*        let loc = self.location(node);*/
        /*let mut children = node.children();*/
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| self.error(node, "name"))
            .and_then(|node| self.parse_ident(&node))?;
        /*let expr = children.next().unwrap();*/
        //let name = self.to_identifier(name)?;
        //let expr = self.to_expr(expr)?;
        let location = self.location(node);
        let def = Definition::new_expr_def(&name, &Some(location));
        Result::Ok(def)
    }

    fn parse_definition(&mut self, node: &Node) -> Result<Definition, Error> {
        match node.kind() {
            "expr_def" => self.parse_expr_def(node),
            _ => Err(self.error(node, "definition")),
        }
    }

    pub fn parse_program(&mut self, node: &Node) -> Result<Program, Errors> {
        match node.kind() {
            "program" => {
                let mut program = Program::empty();
                let mut errors = Errors::new();

                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        match self.parse_definition(&cursor.node()) {
                            Ok(def) => {
                                if errors.is_empty() && program.add_definition(def).is_some() {
                                    panic!("Definition already exist.");
                                }
                            }
                            Err(err) => {
                                errors.add(err);
                            }
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
                if errors.is_empty() {
                    Result::Ok(program)
                } else {
                    Result::Err(errors)
                }
            }
            _ => {
                let mut errors = Errors::new();
                errors.add(self.error(node, "program"));
                Result::Err(errors)
            }
        }
    }
}
