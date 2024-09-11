use super::super::error::Error;
use super::super::location::{Located, Location, Position};
use super::super::stdlib::NAME_ENV;
use super::ast::*;
use super::env::NameEnv;

use tree_sitter::Node;

pub struct Parser {
    file_name: String,
    content: Vec<String>,
    name_env: NameEnv,
}
type ParserResult<T> = Result<(Parser, T), Error>;

const ERROR_KIND: i32 = 201;
const ERROR_KEYWORD: i32 = 202;
const ERROR_OPERATOR: i32 = 203;

impl Parser {
    pub fn make(file_name: &String, content: &[String]) -> Self {
        Self {
            file_name: file_name.to_owned(),
            content: content.to_vec(),
            name_env: NAME_ENV.clone(),
        }
    }

    fn location(&self, node: &Node) -> Location {
        let start = node.start_position();
        let end = node.end_position();
        let pos_start = Position::make(start.row, start.column);
        let pos_end = Position::make(end.row, end.column);
        Location::make(self.file_name.clone(), &self.content, pos_start, pos_end)
    }

    fn error<T>(self, node: &Node, expect: &str, code: i32) -> ParserResult<T> {
        let msg = format!("Expected {expect}");
        Err(Error::error_located(&msg, self.location(node), code))
    }

    fn error_kind<T>(self, node: &Node, expect: &str) -> ParserResult<T> {
        self.error(node, expect, ERROR_KIND)
    }

    fn check_keyword(self, node: &Node, expect: &str) -> ParserResult<()> {
        if node.kind() != expect {
            let expect = format!("keyword '{}'", expect);
            self.error(node, &expect, ERROR_KEYWORD)
        } else {
            Ok((self, ()))
        }
    }

    fn check_operator(self, node: &Node, expect: &str) -> ParserResult<()> {
        if node.kind() != expect {
            let expect = format!("operator '{}'", expect);
            self.error(node, &expect, ERROR_OPERATOR)
        } else {
            Ok((self, ()))
        }
    }

    fn parse_ident(mut self, node: &Node) -> ParserResult<Ident> {
        match node.kind() {
            "ident" => {
                let location = self.location(node);
                let (name_env, ident) = self.name_env.of_location(&location);
                self.name_env = name_env;
                Ok((self, ident.set_location(location)))
            }
            _ => self.error_kind(node, "identifier"),
        }
    }

    fn parse_number_n(self, node: &Node) -> ParserResult<NConst> {
        match node.kind() {
            "number_N" => {
                let location = self.location(node);
                let val = location.text().parse::<NConst>().unwrap();
                Ok((self, val))
            }
            _ => self.error_kind(node, "number"),
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
            _ => self.error_kind(node, "constant"),
        }
    }

    fn parse_expression(mut self, node: &Node) -> ParserResult<WTExpression> {
        let location = self.location(node);
        match node.kind() {
            "constant" => {
                let constant;
                (self, constant) = self.parse_constant(node)?;
                Ok((
                    self,
                    WTExpression::make_constant(constant).set_location(location),
                ))
            }
            _ => self.error_kind(node, "expression"),
        }
    }

    fn parse_ty(mut self, node: &Node) -> ParserResult<Ty> {
        let location = self.location(node);
        match node.kind() {
            "ident" => {
                let ident;
                (self, ident) = self.parse_ident(node)?;
                Ok((self, Ty::make_var(ident).set_location(location)))
            }
            _ => self.error_kind(node, "type"),
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

    fn parse_expr_def(self, node: &Node) -> ParserResult<WTDefinition> {
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
        let def = WTDefinition::make_expr_def(ident, body)
            .set_opt_ty(opt_ty)
            .set_location(location);
        Ok((parser, def))
    }

    fn parse_definition(self, node: &Node) -> ParserResult<WTDefinition> {
        self.parse_expr_def(node)
    }

    /// parse program
    pub fn parse_program(mut self, node: &Node) -> ParserResult<WTProgram> {
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
            _ => self.error_kind(node, "program"),
        }
    }
}
