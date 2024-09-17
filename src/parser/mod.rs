use super::error::Error;
use super::stdlib::NAME_ENV;

pub mod ast;

mod env;
pub use env::*;

mod parse;
pub use parse::*;

mod parse_tree;
pub use parse_tree::*;

/// parse a file
pub fn parse_file(file_name: &str) -> Result<ParseTree<'_>, Error> {
    let language = tree_sitter_start::start_language();
    Ok(ParseTree::of_file(file_name)?
        .set_language(&language)
        .parse())
}

/// make a program with parse tree
pub fn make_program(parse_tree: ParseTree) -> Result<(Parser, ast::WTProgram), Error> {
    let parser = Parser::from_parse_tree(&parse_tree, NAME_ENV.clone());
    let node = parse_tree.get_tree().root_node();
    parser.parse_program(&node).get_result()
}
