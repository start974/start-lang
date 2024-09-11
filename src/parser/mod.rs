pub mod ast;
mod env;
mod parse;
mod parse_tree;
use super::error::Error;
use super::stdlib::NAME_ENV;

pub type Parser<'a> = parse::Parser<'a>;
pub type ParseTree = parse_tree::ParseTree;
pub type NameEnv = env::NameEnv;

/// parse a file
pub fn parse_file(file_name: String) -> Result<ParseTree, Error> {
    ParseTree::of_file(file_name)
}

//pub fn parse_string(file_name: String, content: &String) -> ParseTree {
    //ParseTree::of_string(file_name, content)
//}

/// make a program with parse tree
pub fn make_program(parse_tree: ParseTree) -> Result<ast::WTProgram, Error> {
    let root = parse_tree.root_node();
    let parser = Parser::make(
        parse_tree.file_name(),
        parse_tree.content(),
        NAME_ENV.clone(),
    );
    let res = parser.parse_program(&root);
    res.result()
}
