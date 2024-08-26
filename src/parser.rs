use tree_sitter::{Parser, Tree};

pub struct ParseTree {
    tree: Tree,
    file_name: String,
}

impl ParseTree {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn to_sexp(&self) -> String {
        self.tree.root_node().to_sexp()
    }
}

pub fn parse(file_name: String, input: String) -> ParseTree {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_start::language())
        .expect("Error loading start grammar.");
    let tree = parser.parse(input, None).expect("Parsing error.");
    return ParseTree { tree, file_name };
}

pub fn parse_file(file_name: String) -> ParseTree {
    let file_content = std::fs::read_to_string(file_name.clone()).expect("Failed to read file");
    parse(file_name, file_content)
}
