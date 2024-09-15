pub mod ast;
mod env;
mod parse;
mod parse_tree;

use super::error::Error;
use super::stdlib::NAME_ENV;

pub type Parser<'a> = parse::Parser<'a>;
pub type ParseTree<'a> = parse_tree::ParseTree<'a>;
//pub type ParserResult<'a, T> = parse::ParserResult<'a, T>;
pub type NameEnv = env::NameEnv;

/// parse a file
pub fn parse_file(file_name: &str) -> Result<ParseTree<'_>, Error> {
    let language = tree_sitter_start::start_language();
    Ok(ParseTree::of_file(file_name)?
        .set_language(&language)
        .parse())
}

/// make a program with parse tree
pub fn make_program(parse_tree: ParseTree) -> Result<ast::WTProgram, Error> {
    let parser = Parser::from_parse_tree(&parse_tree, NAME_ENV.clone());
    let node = parse_tree.get_tree().root_node();
    let res = parser.parse_program(&node);
    res.get_res()
}
