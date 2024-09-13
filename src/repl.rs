use super::error::Error;
use super::args::Args;
use super::parser::ast::WTDefsOrExpr;
use super::parser::{ParseTree, Parser};
use super::stdlib::NAME_ENV;
use super::utils::FResult;

use rustyline::DefaultEditor;
use tree_sitter::Node;

struct Env<'a> {
    parse_tree: ParseTree<'a>,
    parser: Parser<'a>,
}

type EnvResult<'a, T> = FResult<Env<'a>, T>;

const FILE_NAME: &str = "<<stdin>>";
const ERROR_EXPR_OR_DEF: i32 = 201;

impl<'a> Env<'a> {
    fn new() -> Self {
        let parser = Parser::make(FILE_NAME, &[], NAME_ENV.clone());
        let language =  tree_sitter_start::start_repl_language();
        let parse_tree = ParseTree::make(FILE_NAME).set_language(&language);
        Self { parse_tree, parser }
    }

/*    fn ok<T>(self, val: T) -> EnvResult<'a, T> {*/
        /*EnvResult::ok(self, val)*/
    /*}*/

    /*fn error<T>(self, err: Error) -> EnvResult<'a, T> {*/
        /*EnvResult::error(self, err)*/
    /*}*/

    /*fn parse_definitions_or_expression(mut self, node: &Node) -> EnvResult<'a, WTDefsOrExpr> {*/
        /*let (parser, res) = self.parser.parse_definitions_or_expression(node).get_pair();*/
        /*self.parser = parser;*/
        /*EnvResult::make(self, res)*/
    /*}*/

    /*fn parse_input(mut self, input: String) -> EnvResult<'a, Node> {*/
        /*self.parse_tree = self.parse_tree.set_content(input);*/
        /*self.parse_tree = self.parse_tree.parse();*/
        /*let node = self.parse_tree.root_node();*/
        /*self.ok(node)*/
    /*}*/

    fn eval(mut self, args: &Args, input: String) -> Self {
        self
        // parse a parse tree
/*        let (env, res) = self.parse_input(input)*/
            /*.inspect(|parse_tree| debug(args.debug_sexp, "SEXP", parse_tree.sexp()))*/
            /*.and_then(|env, node| {*/
                /*env.parse_definitions_or_expression(&node)*/
            /*})*/

            /*.get_pair();*/
        /*match &res {*/
            /*Ok(WTDefsOr::Expr(expr)) => {*/
                /*println!("expr: {expr}")*/
            /*}*/
            /*Ok(WTExprOrDef::Def(def)) => {*/
                /*println!("def: {def}")*/
            /*}*/
            /*Err(err) => {*/
                /*eprintln!("{err}")*/
            /*}*/
        /*}*/
        /*env*/
    }
}

pub fn repl(args: &Args){
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
