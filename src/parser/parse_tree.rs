use crate::error::*;
use std::fs::File;
use std::io::Read;
use tree_sitter::Parser as TSTParser;
use tree_sitter::{Language, Node, Tree};

pub struct ParseTree<'a> {
    file_name: &'a str,
    content: Option<String>,
    parser: TSTParser,
    tree: Option<Tree>,
}

const ERROR_FILE_NOT_FOUND: i32 = 101;
const ERROR_READ: i32 = 102;

impl<'a> ParseTree<'a> {
    // create a new ParseTree
    pub fn make(file_name: &'a str) -> Self {
        Self {
            file_name,
            content: None,
            parser: TSTParser::new(),
            tree: None,
        }
    }

    pub fn set_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self.tree = None;
        self
    }

    /*    pub fn set_file_name(mut self, file_name: &'a str) -> Self {*/
    /*self.file_name = file_name;*/
    /*self*/
    /*}*/

    // create a ParseTree from a file
    pub fn of_file(file_name: &'a str) -> Result<Self, Error> {
        File::open(file_name)
            .map_err(|_| {
                let msg = Head::new().text("No such file").quoted(file_name);
                Error::make(msg, ERROR_FILE_NOT_FOUND)
            })
            .and_then(|mut file| {
                let mut input = String::new();
                match file.read_to_string(&mut input) {
                    Ok(_) => Ok(input),
                    Err(_) => {
                        let msg = Head::new().text("Cannot read file").quoted(file_name);
                        Err(Error::make(msg, ERROR_READ))
                    }
                }
            })
            .map(|content| Self::make(file_name).set_content(content))
    }

    /// set language
    pub fn set_language(mut self, language: &Language) -> Self {
        self.parser
            .set_language(language)
            .expect("Error loading start grammar.");
        self
    }

    /// parse content
    /// can fail if content is not set or parsing fail or content not set
    pub fn parse(mut self) -> Self {
        if self.tree.is_none() {
            let content = self
                .content
                .as_ref()
                .expect("please call ParseTree::set_content");
            let tree = self.parser.parse(content, None).expect("Parsing error.");
            self.tree = Some(tree);
        }
        self
    }

    /// get tree sitter tree
    pub fn get_tree(&self) -> &Tree {
        self.tree.as_ref().expect("please call Parser::parse")
    }

    /// get root node
    pub fn root_node(&self) -> Node {
        self.get_tree().root_node()
    }

    /// sexp of node parsed
    /*    pub fn sexp(&self) -> String {*/
    /*self.root_node().to_sexp()*/
    /*}*/

    /// get file_name
    pub fn get_file_name(&self) -> &'a str {
        self.file_name
    }

    /// get content
    pub fn get_content(&self) -> Vec<String> {
        self.content
            .clone()
            .expect("please call ParseTree::set_content")
            .split('\n')
            .map(|s| s.to_string())
            .collect()
    }
}
