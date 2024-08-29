use super::super::ast::{
    untyped::{Constant, Definition, Expression, Program},
    Env, Ident,
};
use super::super::location::{Located, Location, Position};
use super::error::{Errors, ErrorsResult};
use super::iter_node::IterNode;

use tree_sitter::Node;

pub struct Parser {
    file_name: String,
    content: Vec<String>,
    env: Env,
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

    fn text(&self, &node: &Node) -> String {
        let location = self.location(&node);
        location.text(&self.content)
    }

    fn parse_ident(mut self, node: &Node) -> ParserResult<Ident> {
        match node.kind() {
            "ident" => {
                let location = self.location(node);
                let name = self.text(node);
                let (env, ident) = self.env.make_ident(name);
                self.env = env;
                (self, Ok(ident.set_location(location)))
            }

            _ => self.self_error(node, "identifier"),
        }
    }

    fn parse_n(self, node: &Node) -> ParserResult<Expression> {
        match node.kind() {
            "number_N" => {
                let location = self.location(node);
                let text = self.text(node);
                let val = text.parse::<u32>().unwrap();
                let constant = Constant::make_n(val).set_location(location);
                let expr = Expression::make_constant(constant);
                (self, Ok(expr))
            }
            _ => self.self_error(node, "number"),
        }
    }

    fn parse_expression(self, node: &Node) -> ParserResult<Expression> {
        match node.kind() {
            "number_N" => self.parse_n(node),
            _ => self.self_error(node, "expression"),
        }
    }

    fn parse_expr_def(self, node: &Node) -> ParserResult<Definition> {
        let loc = self.location(node);
        IterNode::new(node, self, ())
            .next(
                &mut |parser, node_def| parser.check_keyword(node_def, "def"),
                &mut |(), ()| (),
            )
            .next(
                &mut |parser, node_ident| parser.parse_ident(node_ident),
                &mut |(), ident| ident,
            )
            // TODO : add ty
            .next(
                &mut |parser, node_eq_def| parser.check_keyword(node_eq_def, ":="),
                &mut |ident, ()| ident,
            )
            .next(
                &mut |parser, node_expr| parser.parse_expression(node_expr),
                &mut |ident, expr| (ident, expr),
            )
            .map_result(|(ident, expr)| Definition::make_expr_def(ident, expr).set_location(loc))
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
