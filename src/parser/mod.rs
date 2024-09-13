pub mod ast;
mod env;
mod parse;
mod parse_tree;

use super::error::Error;
use super::stdlib::NAME_ENV;

pub type Parser<'a> = parse::Parser<'a>;
pub type ParseTree<'a> = parse_tree::ParseTree<'a>;
pub type ParserResult<'a, T> = parse::ParserResult<'a, T>;
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
    let node = parse_tree.get_tree().root_node();
    let file_name = parse_tree.get_file_name();
    let content = parse_tree.get_content();
    let parser = Parser::make(file_name, &content, NAME_ENV.clone());
    let res = parser.parse_program(&node);
    res.get_res()
}
