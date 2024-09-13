use super::args::Args;
use super::parser::ast::WTDefsOrExpr;
use super::parser::{ParseTree, Parser};
use super::stdlib::NAME_ENV;
use super::utils::debug::debug_sexp;
use super::utils::FResult;

use rustyline::DefaultEditor;

struct Env {
    parse_tree: ParseTree<'static>,
    parser: Parser<'static>,
}

const FILE_NAME: &str = "<<stdin>>";

impl Env {
    fn new() -> Self {
        let language = tree_sitter_start::start_repl_language();
        let parse_tree = ParseTree::make(FILE_NAME).set_language(&language);
        let parser = Parser::make(FILE_NAME, NAME_ENV.clone());
        Self { parse_tree, parser }
    }

    fn eval(mut self, args: &Args, input: String) -> Self {
        // make a part tree
        self.parse_tree = self.parse_tree.set_content(input).parse();
        debug_sexp(args, &self.parse_tree);

        // parse
        let content = self.parse_tree.get_content();
        let node = self.parse_tree.root_node();
        let (parser, res) = self
            .parser
            .set_content(&content)
            .parse_definitions_or_expression(&node)
            .get_pair();
        self.parser = parser;

        match &res {
            Ok(WTDefsOrExpr::Expression(expr)) => {
                println!("expr: {expr}")
            }
            Ok(WTDefsOrExpr::Definitions(prog)) => {
                println!("def: {prog}")
            }
            Err(err) => {
                eprintln!("{err}")
            }
        }
        self
    }
}

pub fn repl(args: &Args) {
    let mut env = Env::new();
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        match rl.readline(">> ") {
            Ok(mut line) => {
                while !line.ends_with(".") {
                    line.pop();
                    match &rl.readline("-- ") {
                        Ok(line2) => {
                            line += "\n";
                            line += line2
                        }
                        Err(_) => return,
                    }
                }
                // interpret
                env = env.eval(args, line)
            }
            Err(_) => return,
        }
    }
}
