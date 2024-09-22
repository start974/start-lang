use super::ast::*;
use super::env::NameEnv;
use super::parse_tree::ParseTree;
use crate::error::Error;
use crate::error::*;
use crate::location::{Located, Location, Position};
use crate::utils::colored::*;
use crate::utils::FResult;
use tree_sitter::Node;

pub struct Parser<'a> {
    file_name: &'a str,
    content: Vec<String>,
    name_env: NameEnv,
}

pub type ParserResult<'a, T, E> = FResult<Parser<'a>, T, E>;

const ERROR_KIND: i32 = 201;
const ERROR_KEYWORD: i32 = 202;
const ERROR_OPERATOR: i32 = 203;
const ERROR_WILDCARD: i32 = 204;

impl<'a> Parser<'a> {
    /// make a parser
    pub fn make(file_name: &'a str, name_env: NameEnv) -> Self {
        Self {
            content: Vec::new(),
            file_name,
            name_env,
        }
    }

    pub fn from_parse_tree(parse_tree: &ParseTree<'a>, name_env: NameEnv) -> Self {
        let file_name = parse_tree.get_file_name();
        let content = parse_tree.get_content();
        Self {
            content,
            file_name,
            name_env,
        }
    }

    /// set content of parser
    pub fn set_content(mut self, content: &[String]) -> Self {
        self.content = content.to_vec();
        self
    }

    fn location(&self, node: &Node) -> Location {
        if self.content.is_empty() {
            panic!("content is empty please set content with Parser::set_content")
        }

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

    fn ok<T, E>(self, val: T) -> ParserResult<'a, T, E> {
        ParserResult::ok(self, val)
    }

    fn error<T>(self, node: &Node, err: Error) -> ParserResult<'a, T, Error> {
        let location = self.location(node);
        let err = err.set_location(location);
        ParserResult::err(self, err)
    }

    fn error_kind<T>(self, node: &Node, expect: &str) -> ParserResult<'a, T, Error> {
        let msg = Head::new().text("Expected").important(expect);
        self.error(node, Error::make(msg, ERROR_KIND))
    }

    fn error_wilcard<T>(self, node: &Node) -> ParserResult<'a, T, Error> {
        let msg = Head::new()
            .text("Not allowed to use wildcard")
            .quoted("_")
            .text("as an expression");
        self.error(node, Error::make(msg, ERROR_WILDCARD))
    }

    fn error_keyword<T>(self, node: &Node, expect: &str) -> ParserResult<'a, T, Error> {
        let msg = Head::new().text("Expected keyword").quoted(expect);
        self.error(node, Error::make(msg, ERROR_KEYWORD))
    }

    fn error_operator<T>(self, node: &Node, expect: &str) -> ParserResult<'a, T, Error> {
        let msg = Head::new().text("Expected operator").quoted(expect);
        self.error(node, Error::make(msg, ERROR_OPERATOR))
    }

    fn check<F>(self, node: &Node, expect: &str, f: F) -> ParserResult<'a, (), Error>
    where
        F: FnOnce(Parser<'a>, &Node, &str) -> ParserResult<'a, (), Error>,
    {
        if node.kind() != expect {
            f(self, node, expect)
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

    fn parse_ident(mut self, node: &Node) -> ParserResult<'a, Ident, Error> {
        match node.kind() {
            "ident" => {
                let location = self.location(node);
                let (name_env, ident) = self.name_env.of_location(&location);
                self.name_env = name_env;
                self.ok(ident)
            }
            _ => self.error_kind(node, "identifier"),
        }
    }

    fn parse_number_n(self, node: &Node) -> ParserResult<'a, NConst, Error> {
        match node.kind() {
            "number_N" => {
                let location = self.location(node);
                let txt = location.text();
                let val = if txt.starts_with("0b") || txt.starts_with("0B") {
                    NConst::parse_bytes(txt[2..].as_bytes(), 2)
                } else if txt.starts_with("0o") || txt.starts_with("0O") {
                    NConst::parse_bytes(txt[2..].as_bytes(), 8)
                } else if txt.starts_with("0x") || txt.starts_with("0X") {
                    NConst::parse_bytes(txt[2..].as_bytes(), 16)
                } else {
                    NConst::parse_bytes(txt.as_bytes(), 10)
                }
                .unwrap();
                self.ok(val)
            }
            _ => self.error_kind(node, "number"),
        }
    }

    fn parse_constant(self, node: &Node) -> ParserResult<'a, Constant, Error> {
        let child = node.child(0).unwrap();
        match child.kind() {
            "number_N" => self.parse_number_n(&child).map_res(Constant::make_n),
            _ => self.error_kind(node, "constant"),
        }
    }

    pub fn parse_expression(self, node: &Node) -> ParserResult<'a, WTExpression, Error> {
        match node.kind() {
            "constant" => self
                .parse_constant(node)
                .map_res(WTExpression::make_constant)
                .map_res2(|parser, constant| parser.set_location(node, constant)),
            "ident" => self
                .parse_ident(node)
                .and_then(|parser, ident| {
                    if ident.name == "_" {
                        parser.error_wilcard(node)
                    } else {
                        parser.ok(WTExpression::make_var(ident))
                    }
                })
                .map_res2(|parser, ident| parser.set_location(node, ident)),
            _ => self.error_kind(node, "expression"),
        }
    }

    fn parse_ty(self, node: &Node) -> ParserResult<'a, Ty, Error> {
        match node.kind() {
            "ident" => self
                .parse_ident(node)
                .map_res(Ty::make_var)
                .map_res2(|parser, ident| parser.set_location(node, ident)),
            _ => self.error_kind(node, "type"),
        }
    }

    fn parse_ty_restr(self, node: &Node) -> ParserResult<'a, Ty, Error> {
        let mut child = node.child(0).unwrap();
        // colon
        self.check(&child, ":", Self::error_operator)
            .and_then(|parser, ()| {
                child = child.next_sibling().unwrap();
                parser.parse_ty(&child)
            })
    }

    fn parse_expr_def(self, node: &Node) -> ParserResult<'a, WTExprDef, Error> {
        let mut child = node.child(0).unwrap();
        // def keyword
        self.check(&child, "def", Self::error_keyword)
            // identifier
            .and_then(|parser, ()| {
                child = child.next_sibling().unwrap();
                let location = parser.location(&child);
                parser
                    .parse_ident(&child)
                    .map_res(|ident| (ident, location))
            })
            // type restriction
            .and_then(|parser, (ident, location)| {
                child = child.next_sibling().unwrap();
                if child.grammar_name() == "ty_restr" {
                    let res = parser.parse_ty_restr(&child);
                    child = child.next_sibling().unwrap();
                    res.map_res(|ty| (ident, location, Some(ty)))
                } else {
                    parser.ok((ident, location, None))
                }
            })
            // eq def
            .and_then(|parser, old_res| {
                parser
                    .check(&child, ":=", Self::error_operator)
                    .map_res(|()| old_res)
            })
            // body
            .and_then(|parser, (ident, location, opt_ty)| {
                child = child.next_sibling().unwrap();
                parser
                    .parse_expression(&child)
                    .map_res(|body| (ident, location, opt_ty, body))
            })
            .map_res(|(name, location, opt_ty, body)| {
                WTExprDef::new(name, body)
                    .set_opt_ty(opt_ty)
                    .set_location(location)
            })
    }

    fn parse_type_def(self, node: &Node) -> ParserResult<'a, TyDef, Error> {
        let mut child = node.child(0).unwrap();
        // type keyword
        self.check(&child, "type", Self::error_keyword)
            // identifier
            .and_then(|parser, ()| {
                child = child.next_sibling().unwrap();
                parser.parse_ident(&child)
            })
            // eq def
            .and_then(|parser, ident| {
                child = child.next_sibling().unwrap();
                parser
                    .check(&child, ":=", Self::error_operator)
                    .map_res(|()| ident)
            })
            // body
            .and_then(|parser, ident| {
                child = child.next_sibling().unwrap();
                parser.parse_ty(&child).map_res(|ty| (ident, ty))
            })
            .map_res(|(name, ty)| TyDef::new(name, ty))
            .map_res2(|parser, def| parser.set_location(node, def))
    }

    fn parse_definition(self, node: &Node) -> ParserResult<'a, WTDefinition, Error> {
        match node.child(0) {
            Some(keyword) if keyword.kind() == "def" => {
                self.parse_expr_def(node).map_res(WTDefinition::ExprDef)
            }
            Some(keyword) if keyword.kind() == "type" => {
                self.parse_type_def(node).map_res(WTDefinition::TyDef)
            }
            _ => self.error_kind(node, "definition"),
        }
    }

    /// parse program
    pub fn parse_program(self, node: &Node) -> ParserResult<'a, WTProgram, Errors> {
        match node.kind() {
            "program" => {
                let mut res = self.ok(WTProgram::empty());
                for i in 0..node.child_count() {
                    let child = node.child(i).unwrap();
                    res = res.combine(
                        |parser| parser.parse_definition(&child),
                        WTProgram::add_definition,
                    );
                }
                res
            }
            _ => self.error_kind(node, "program").to_errors(),
        }
    }

    /// parse definitions
    pub fn parse_repl_definitions(self, node: &Node) -> ParserResult<'a, WTProgram, Errors> {
        match node.kind() {
            "definitions" => {
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
            _ => self.error_kind(node, "program").to_errors(),
        }
    }

    pub fn parse_repl_expression(self, node: &Node) -> ParserResult<'a, WTExpression, Error> {
        match node.kind() {
            "expression" => {
                let child = node.child(0).unwrap();
                self.parse_expression(&child)
            }
            _ => self.error_kind(node, "expression"),
        }
    }

    pub fn parse_definitions_or_expression(
        self,
        node: &Node,
    ) -> ParserResult<'a, WTDefsOrExpr, Errors> {
        if node.kind() == "definitions_or_expression" {
            let child = node.child(0).unwrap();
            match child.kind() {
                "definitions" => self
                    .parse_repl_definitions(&child)
                    .map_res(WTDefsOrExpr::Definitions),
                "expression" => self
                    .parse_repl_expression(&child)
                    .map_res(WTDefsOrExpr::Expression)
                    .to_errors(),
                _ => self
                    .error_kind(node, "definitions or expression")
                    .to_errors(),
            }
        } else {
            self.error_kind(node, "definitions or expression")
                .to_errors()
        }
    }
}

impl<'a> std::fmt::Display for Parser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Parser {}", self.file_name)?;
        writeln!(f, "Name_env:\n{}", self.name_env)
    }
}

impl<'a> Colored for Parser<'a> {
    fn colored(&self) -> String {
        let mut res = cformat!("<bold>Parser</bold> <blue>{}</blue>\n", self.file_name);
        res += &cformat!("<bold>Name_env:</bold>\n");
        res += &self.name_env.colored();
        res
    }
}
