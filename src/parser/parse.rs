use super::super::error::Error;
use super::super::location::{Located, Location, Position};
use super::ast::{Constant, Definition, Env, Expression, Ident, Program, Ty};

use tree_sitter::Node;

pub struct Parser {
    file_name: String,
    content: Vec<String>,
    env: Env,
}
type ParserResult<T> = Result<(Parser, T), Error>;

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

    fn error<T>(self, node: &Node, expect: &str) -> ParserResult<T> {
        let msg = format!("Expected {expect}");
        Err(Error::error_located(&msg, self.location(node)))
    }

    fn check_keyword(self, node: &Node, expect: &str) -> ParserResult<()> {
        if node.kind() != expect {
            let expect = format!("keyword '{}'", expect);
            self.error(node, &expect)
        } else {
            Ok((self, ()))
        }
    }

    fn check_operator(self, node: &Node, expect: &str) -> ParserResult<()> {
        if node.kind() != expect {
            let expect = format!("operator '{}'", expect);
            self.error(node, &expect)
        } else {
            Ok((self, ()))
        }
    }

    fn parse_ident(mut self, node: &Node) -> ParserResult<Ident> {
        match node.kind() {
            "ident" => {
                let location = self.location(node);
                let (env, ident) = self.env.of_location(&location);
                self.env = env;
                Ok((self, ident.set_location(location)))
            }
            _ => self.error(node, "identifier"),
        }
    }

    fn parse_number_n(self, node: &Node) -> ParserResult<u32> {
        match node.kind() {
            "number_N" => {
                let location = self.location(node);
                let val = location.text().parse::<u32>().unwrap();
                Ok((self, val))
            }
            _ => self.error(node, "number"),
        }
    }

    fn parse_constant(mut self, node: &Node) -> ParserResult<Constant> {
        let child = node.child(0).unwrap();
        match child.kind() {
            "number_N" => {
                let n;
                (self, n) = self.parse_number_n(&child)?;
                Ok((self, Constant::make_n(n)))
            }
            _ => self.error(node, "constant"),
        }
    }

    fn parse_expression(mut self, node: &Node) -> ParserResult<Expression> {
        match node.kind() {
            "constant" => {
                let constant;
                (self, constant) = self.parse_constant(node)?;
                Ok((self, Expression::make_constant(constant)))
            }
            _ => self.error(node, "expression"),
        }
    }

    fn parse_ty(mut self, node: &Node) -> ParserResult<Ty> {
        match node.kind() {
            "ident" => {
                let ident;
                (self, ident) = self.parse_ident(node)?;
                Ok((self, Ty::make_var(ident)))
            }
            _ => self.error(node, "type"),
        }
    }

    fn parse_ty_restr(self, node: &Node) -> ParserResult<Ty> {
        let parser = self;

        let mut node = node.child(0).unwrap();
        // colon
        let (parser, ()) = parser.check_operator(&node, ":")?;

        // ty
        node = node.next_sibling().unwrap();
        parser.parse_ty(&node)
    }

    fn parse_expr_def(self, node: &Node) -> ParserResult<Definition> {
        // get location
        let location = self.location(node);

        let parser = self;

        // definition node
        let mut node = node.child(0).unwrap();
        let (parser, ()) = parser.check_keyword(&node, "def")?;

        // identifier
        node = node.next_sibling().unwrap();
        let (parser, ident) = parser.parse_ident(&node)?;

        // type restriction
        node = node.next_sibling().unwrap();
        let (parser, opt_ty) = if node.grammar_name() == "ty_restr" {
            let (parser, ty) = parser.parse_ty_restr(&node)?;
            node = node.next_sibling().unwrap();
            (parser, Some(ty))
        } else {
            (parser, None)
        };

        // eq def
        let (parser, ()) = parser.check_operator(&node, ":=")?;

        // body
        node = node.next_sibling().unwrap();
        let (parser, body) = parser.parse_expression(&node)?;

        // make definition
        let def = Definition::make_expr_def(ident, body)
            .set_opt_ty(opt_ty)
            .set_location(location);
        Ok((parser, def))
    }

    fn parse_definition(self, node: &Node) -> ParserResult<Definition> {
        self.parse_expr_def(node)
    }

    /// parse program
    pub fn parse_program(mut self, node: &Node) -> ParserResult<Program> {
        match node.kind() {
            "program" => {
                let mut res = Ok(Program::empty());
                let content = self.content.clone();
                let file_name = self.file_name.clone();
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();

                    match (self.parse_definition(&child), res) {
                        (Ok((self2, def)), Ok(program)) => {
                            self = self2;
                            res = Ok(program.add_definition(def));
                        }
                        (Err(e), Ok(_)) | (Ok(_), Err(e)) => {
                            self = Self::make(&file_name, &content);
                            res = Err(e);
                        }
                        (Err(e2), Err(e1)) => {
                            self = Self::make(&file_name, &content);
                            res = Err(e1.error_add(e2));
                        }
                    };
                }
                res.map(|prog| (self, prog))
            }
            _ => self.error(node, "program"),
        }
    }
}
