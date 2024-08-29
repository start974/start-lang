use super::super::ast::untyped::Program;
use super::error::Errors;
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
    pub fn from_string(file_name: String, input: &String) -> Self {
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

    pub fn from_file(file_name: String) -> Self {
        let mut file = match File::open(file_name.clone()) {
            Result::Ok(file) => file,
            Result::Err(_) => panic!("No such file {}.", file_name),
        };

        let mut input = String::new();
        file.read_to_string(&mut input).expect("failed to read!");
        Self::from_string(file_name, &input)
    }

    pub fn to_sexp(&self) -> String {
        self.tree.root_node().to_sexp()
    }

    // make a parseTree
    pub fn to_program(&self) -> Result<Program, Errors> {
        let root = self.tree.root_node();
        let parser = Parser::make(&self.file_name, &self.content);
        let (_, res) = parser.parse_program(&root);
        res
    }
}
