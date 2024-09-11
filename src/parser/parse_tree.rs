use super::super::error::Error;

use std::fs::File;
use std::io::Read;
use tree_sitter::Parser as TSTParser;
use tree_sitter::Tree;

pub struct ParseTree<'a> {
    tree: Tree,
    file_name: &'a str,
    content: Vec<String>,
}

const ERROR_FILE_NOT_FOUND: i32 = 101;
const ERROR_READ: i32 = 102;

impl<'a> ParseTree<'a> {
    pub fn of_string(file_name: &'a str, input: &String) -> Self {
        let mut parser = TSTParser::new();
        parser
            .set_language(&tree_sitter_start::language())
            .expect("Error loading start grammar.");
        let tree = parser.parse(input, None).expect("Parsing error.");
        let content = input.split('\n').map(str::to_string).collect();
        ParseTree {
            tree,
            file_name,
            content,
        }
    }

    pub fn of_file(file_name: &'a str) -> Result<Self, Error> {
        File::open(file_name)
            .map_err(|_| {
                let msg = format!("No such file '{file_name}'");
                Error::error_simple(&msg, ERROR_FILE_NOT_FOUND)
            })
            .and_then(|mut file| {
                let mut input = String::new();
                match file.read_to_string(&mut input) {
                    Ok(_) => Ok(input),
                    Err(_) => {
                        let msg = format!("Cannot read file '{file_name}'.");
                        Err(Error::error_simple(&msg, ERROR_READ))
                    }
                }
            })
            .map(|input| Self::of_string(file_name, &input))
    }

    /// root node of tree
    pub fn root_node(&self) -> tree_sitter::Node {
        self.tree.root_node()
    }

    /// get file_name
    pub fn file_name(&self) -> &'a str {
        self.file_name
    }

    /// get content
    pub fn content(&self) -> &Vec<String> {
        &self.content
    }
}

impl<'a> std::fmt::Display for ParseTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.root_node().to_sexp())
    }
}
