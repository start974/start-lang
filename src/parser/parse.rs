use super::super::error::Error;
use super::super::location::{Located, Location, Position};
use super::ast::{Constant, Definition, Env, Expression, Ident, Program, Ty};
use super::iter_node::IterNode;

use tree_sitter::Node;

pub struct Parser {
    file_name: String,
    content: Vec<String>,
    env: Env,
}
type ParserResult<T> = (Parser, Result<T, Error>);

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
        let pos_start = Position::make(start.row, start.column);
        let pos_end = Position::make(end.row, end.column);
        Location::make(self.file_name.clone(), &self.content, pos_start, pos_end)
    }

    fn error(&self, node: &Node, expect: &str) -> Error {
        let msg = format!("Expected {expect}");
        Error::error_located(&msg, self.location(node))
    }

    fn self_error_res<T>(self, node: &Node, expect: &str) -> ParserResult<T> {
        let error = self.error(node, expect);
        (self, Err(error))
    }

    fn check_keyword(self, node: &Node, expect: &str) -> ParserResult<()> {
        if node.kind() != expect {
            let expect = format!("'{}' keyword", expect);
            self.self_error_res(node, &expect)
        } else {
            (self, Ok(()))
        }
    }

    fn parse_ident(mut self, node: &Node) -> ParserResult<Ident> {
        match node.kind() {
            "ident" => {
                let location = self.location(node);
                let (env, ident) = self.env.of_location(&location);
                self.env = env;
                (self, Ok(ident.set_location(location)))
            }
            _ => self.self_error_res(node, "identifier"),
        }
    }

    fn parse_number_n(self, node: &Node) -> ParserResult<u32> {
        match node.kind() {
            "number_N" => {
                let location = self.location(node);
                let val = location.text().parse::<u32>().unwrap();
                (self, Ok(val))
            }
            _ => self.self_error_res(node, "number"),
        }
    }

    fn parse_const(self, node: &Node) -> ParserResult<Constant> {
        let err = self.error(node, "constant");
        IterNode::new(node, self, ())
            .apply(
                &mut |parser, node_n| parser.parse_number_n(node_n),
                &mut |(), n| n,
            )
            .map(Constant::make_n)
            .map_error(|_| err)
            .acc_result()
    }

    fn parse_expression(self, node: &Node) -> ParserResult<Expression> {
        let err = self.error(node, "expression");
        IterNode::new(node, self, ())
            .apply(
                &mut |parser, node_n| parser.parse_const(node_n),
                &mut |(), n| n,
            )
            .map(Expression::make_constant)
            .map_error(|_| err)
            .acc_result()
    }

    fn parse_ty(self, node: &Node) -> ParserResult<Ty> {
        let err = self.error(node, "type");
        IterNode::new(node, self, ())
            .apply(
                &mut |parser, node_ident| parser.parse_ident(node_ident),
                &mut |(), ident| ident,
            )
            .map(Ty::make_var)
            .map_error(|_| err)
            .acc_result()
    }

    fn parse_ty_restr(self, node: &Node) -> ParserResult<Ty> {
        match node.kind() {
            "ty_restr" => IterNode::new(node, self, ())
                .first_child()
                .apply_next(
                    &mut |parser, node_semi_col| parser.check_keyword(node_semi_col, ":"),
                    &mut |(), ()| (),
                )
                .apply(
                    &mut |parser, node_ty| parser.parse_ty(node_ty),
                    &mut |(), ty| ty,
                )
                .acc_result(),
            _ => self.self_error_res(node, "type restriction"),
        }
    }

    fn parse_expr_def(self, node: &Node) -> ParserResult<Definition> {
        let loc = self.location(node);
        IterNode::new(node, self, ())
            .first_child()
            .apply_next(
                &mut |parser, node_def| parser.check_keyword(node_def, "def"),
                &mut |(), ()| (),
            )
            .apply_next(
                &mut |parser, node_ident| parser.parse_ident(node_ident),
                &mut |(), ident| ident,
            )
            .apply_opt_next(
                &mut |parser, node_ty| parser.parse_ty_restr(node_ty),
                &mut |ident, opt_ty| (ident, opt_ty),
            )
            .apply_next(
                &mut |parser, node_eq_def| parser.check_keyword(node_eq_def, ":="),
                &mut |(ident, opt_ty), ()| (ident, opt_ty),
            )
            .apply(
                &mut |parser, node_expr| parser.parse_expression(node_expr),
                &mut |(ident, opt_ty), expr| (ident, opt_ty, expr),
            )
            .map(|(ident, opt_ty, expr)| {
                Definition::make_expr_def(ident, expr)
                    .set_opt_ty(opt_ty)
                    .set_location(loc)
            })
            .acc_result()
    }

    fn parse_definition(self, node: &Node) -> ParserResult<Definition> {
        self.parse_expr_def(node)
    }

    /// parse program
    pub fn parse_program(self, node: &Node) -> ParserResult<Program> {
        match node.kind() {
            "program" => IterNode::new(node, self, Program::empty())
                .first_child()
                .repeat(
                    &mut |parser, node_def| parser.parse_definition(node_def),
                    &mut |program, definition| program.add_definition(definition),
                )
                .acc_result(),
            _ => self.self_error_res(node, "program"),
        }
    }
}
