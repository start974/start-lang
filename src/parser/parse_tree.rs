use super::super::error::Error;
use super::ast::Program;
use super::parse::Parser;

use std::fs::File;
use std::io::Read;
use tree_sitter::Parser as TSTParser;
use tree_sitter::Tree;

pub struct ParseTree {
    tree: Tree,
    file_name: String,
    content: Vec<String>,
}

impl ParseTree {
    pub fn of_string(file_name: String, input: &String) -> Self {
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

    pub fn of_file(file_name: String) -> Result<Self, Error> {
        File::open(file_name.clone())
            .map_err(|_| {
                let msg = format!("No such file '{file_name}'.");
                Error::error_simple(&msg)
            })
            .and_then(|mut file| {
                let mut input = String::new();
                match file.read_to_string(&mut input) {
                    Ok(_) => Ok(input),
                    Err(_) => {
                        let msg = format!("Cannot read file '{file_name}'.");
                        Err(Error::error_simple(&msg))
                    }
                }
            })
            .map(|input| Self::of_string(file_name, &input))
    }

    pub fn to_sexp(&self) -> String {
        self.tree.root_node().to_sexp()
    }

    // make a parseTree
    pub fn to_program(&self) -> Result<Program, Error> {
        let root = self.tree.root_node();
        let parser = Parser::make(&self.file_name, &self.content);
        let (_, res) = parser.parse_program(&root);
        res
    }
}
