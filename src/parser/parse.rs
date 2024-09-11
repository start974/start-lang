use super::super::error::Error;
use super::super::location::{Located, Location, Position};
use super::super::utils::FResult;
use super::ast::*;
use super::env::NameEnv;

use tree_sitter::Node;

pub struct Parser<'a> {
    file_name: &'a str,
    content: Vec<String>,
    name_env: NameEnv,
}

pub type ParserResult<'a, T> = FResult<Parser<'a>, T>;

const ERROR_KIND: i32 = 201;
const ERROR_KEYWORD: i32 = 202;
const ERROR_OPERATOR: i32 = 203;

impl<'a> Parser<'a> {
    pub fn make(file_name: &'a str, content: &[String], name_env: NameEnv) -> Self {
        Self {
            content: content.to_vec(),
            file_name,
            name_env,
        }
    }

    /// update content
    //pub fn update(mut self, file_name: &'a str, content: &[String]) -> Self {
    //self.file_name = file_name;
    //self.content = content.to_vec();
    //self
    //}

    fn location(&self, node: &Node) -> Location {
        let start = node.start_position();
        let end = node.end_position();
        let pos_start = Position::make(start.row, start.column);
        let pos_end = Position::make(end.row, end.column);
        Location::make(
            self.file_name.to_string(),
            &self.content,
            pos_start,
            pos_end,
        )
    }

    fn error<T>(self, node: &Node, expect: &str, code: i32) -> ParserResult<'a, T> {
        let msg = format!("Expected {expect}");
        let location = self.location(node);
        let err = Error::error_located(&msg, location, code);
        ParserResult::<T>::error(self, err)
    }

    fn ok<T>(self, val: T) -> ParserResult<'a, T> {
        ParserResult::ok(self, val)
    }

    fn error_kind<T>(self, node: &Node, expect: &str) -> ParserResult<'a, T> {
        self.error(node, expect, ERROR_KIND)
    }

    fn check_keyword(self, node: &Node, expect: &str) -> ParserResult<'a, ()> {
        if node.kind() != expect {
            let expect = format!("keyword '{}'", expect);
            self.error(node, &expect, ERROR_KEYWORD)
        } else {
            self.ok(())
        }
    }

    fn check_operator(self, node: &Node, expect: &str) -> ParserResult<'a, ()> {
        if node.kind() != expect {
            let expect = format!("operator '{}'", expect);
            self.error(node, &expect, ERROR_OPERATOR)
        } else {
            self.ok(())
        }
    }

     fn set_location<T>(&self, node: &Node, val: T) -> T
    where
        T: Located,
    {
        let location = self.location(node);
        val.set_location(location)
    }

    fn parse_ident(mut self, node: &Node) -> ParserResult<'a, Ident> {
        match node.kind() {
            "ident" => {
                let location = self.location(node);
                let (name_env, ident) = self.name_env.of_location(&location);
                self.name_env = name_env;
                let ident = self.set_location(node, ident);
                self.ok(ident)
            }
            _ => self.error_kind(node, "identifier"),
        }
    }

    fn parse_number_n(self, node: &Node) -> ParserResult<'a, NConst> {
        match node.kind() {
            "number_N" => {
                let location = self.location(node);
                let val = location.text().parse::<NConst>().unwrap();
                self.ok(val)
            }
            _ => self.error_kind(node, "number"),
        }
    }

    fn parse_constant(self, node: &Node) -> ParserResult<'a, Constant> {
        let child = node.child(0).unwrap();
        match child.kind() {
            "number_N" => self.parse_number_n(&child).map_res(Constant::make_n),
            _ => self.error_kind(node, "constant"),
        }
    }

    pub fn parse_expression(self, node: &Node) -> ParserResult<'a, WTExpression> {
        match node.kind() {
            "constant" => self
                .parse_constant(node)
                .map_res(WTExpression::make_constant)
                .map_res2(|parser, constant| parser.set_location(node, constant)),
            _ => self.error_kind(node, "expression"),
        }
    }

    fn parse_ty(self, node: &Node) -> ParserResult<'a, Ty> {
        match node.kind() {
            "ident" => self
                .parse_ident(node)
                .map_res(Ty::make_var)
                .map_res2(|parser, ident| parser.set_location(node, ident)),
            _ => self.error_kind(node, "type"),
        }
    }

    fn parse_ty_restr(self, node: &Node) -> ParserResult<'a, Ty> {
        let mut child = node.child(0).unwrap();
        // colon
        self.check_operator(&child, ":").and_then(|parser, ()| {
            child = child.next_sibling().unwrap();
            parser.parse_ty(&child)
        })
    }

    fn parse_expr_def(self, node: &Node) -> ParserResult<'a, WTDefinition> {
        let mut child = node.child(0).unwrap();
        // definition child
        self.check_keyword(&child, "def")
            // identifier
            .and_then(|parser, ()| {
                child = child.next_sibling().unwrap();
                parser.parse_ident(&child)
            })
            // type restriction
            .and_then(|parser, ident| {
                child = child.next_sibling().unwrap();
                if child.grammar_name() == "ty_restr" {
                    let res = parser.parse_ty_restr(&child);
                    child = child.next_sibling().unwrap();
                    res.map_res(|ty| (ident, Some(ty)))
                } else {
                    parser.ok((ident, None))
                }
            })
            // eq def
            .and_then(|parser, old_res| parser.check_operator(&child, ":=").map_res(|()| old_res))
            // body
            .and_then(|parser, (ident, opt_ty)| {
                child = child.next_sibling().unwrap();
                parser
                    .parse_expression(&child)
                    .map_res(|body| (ident, opt_ty, body))
            })
            .map_res(|(name, opt_ty, body)| {
                WTDefinition::make_expr_def(name, body).set_opt_ty(opt_ty)
            })
            .map_res2(|parser, def| parser.set_location(node, def))
    }

    pub fn parse_definition(self, node: &Node) -> ParserResult<'a, WTDefinition> {
        self.parse_expr_def(&node)
    }

    /// parse program
    pub fn parse_program(self, node: &Node) -> ParserResult<'a, WTProgram> {
        match node.kind() {
            "program" => {
                let mut res = self.ok(Program::empty());
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    res = res.combine(
                        |parser| parser.parse_definition(&child),
                        Program::add_definition,
                    );
                }
                res
            }
            _ => self.error_kind(node, "program"),
        }
    }
}
