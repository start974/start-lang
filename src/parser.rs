mod parse;
mod parse_tree;
mod env;
pub mod ast;
use super::error::Error;

pub type Parser = parse::Parser;
pub type ParseTree = parse_tree::ParseTree;
pub type NameEnv = env::NameEnv;

/// parse a file
pub fn parse_file(file_name: String) -> Result<ParseTree, Error> {
    ParseTree::of_file(file_name)
}

/// make a program with parse tree
pub fn make_program(parse_tree : ParseTree) -> Result<ast::WTProgram, Error> {
    let root = parse_tree.root_node();
    let parser = Parser::make(parse_tree.file_name(), parse_tree.content());
    let (_, prog) = parser.parse_program(&root)?;
    Ok(prog)
}
