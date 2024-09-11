pub mod ast;
mod env;
mod parse;
mod parse_tree;

use super::error::Error;
use super::stdlib::NAME_ENV;

pub type Parser<'a> = parse::Parser<'a>;
pub type ParseTree<'a> = parse_tree::ParseTree<'a>;
pub type NameEnv = env::NameEnv;

/// parse a file
pub fn parse_file(file_name: &str) -> Result<ParseTree<'_>, Error> {
    ParseTree::of_file(file_name)
}

/// make a program with parse tree
pub fn make_program(parse_tree: ParseTree) -> Result<ast::WTProgram, Error> {
    let root = parse_tree.root_node();
    let parser = Parser::make(
        parse_tree.file_name(),
        parse_tree.content(),
        NAME_ENV.clone(),
    );
    let res = parser.parse_program(&root);
    res.get_res()
}
