use crate::args::Args;
use crate::interpreter::Context;
use crate::parser::{ast::WTDefsOrExpr, ParseTree, Parser};
use crate::stdlib::{NAME_ENV, TYPE_ENV};
use crate::typing::{
    ast::{TDefsOrExpr, Typed},
    Typer,
};
use crate::utils::{debug::debug_sexp, FResult};

use rustyline::DefaultEditor;

struct Env {
    parse_tree: ParseTree<'static>,
    parser: Parser<'static>,
    typer: Typer,
    interpreter: Context,
}

const FILE_NAME: &str = "<<stdin>>";

type EnvResult<T> = FResult<Env, T>;

impl Env {
    fn new() -> Self {
        let language = tree_sitter_start::start_repl_language();
        let parse_tree = ParseTree::make(FILE_NAME).set_language(&language);
        let parser = Parser::make(FILE_NAME, NAME_ENV.clone());
        let typer = Typer::make(TYPE_ENV.clone());
        let interpreter = Context::empty();
        Self {
            parse_tree,
            parser,
            typer,
            interpreter,
        }
    }

    fn parse(mut self, args: &Args, input: String) -> EnvResult<WTDefsOrExpr> {
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
        EnvResult::make(self, res)
    }

    fn typing(mut self, defs_or_expr: &WTDefsOrExpr) -> EnvResult<TDefsOrExpr> {
        let (typer, res) = self
            .typer
            .type_definitions_or_expression(defs_or_expr)
            .get_pair();
        self.typer = typer;
        EnvResult::make(self, res)
    }

    fn eval(mut self, args: &Args, input: String) -> Self {
        let (env, res) = self
            .parse(args, input)
            .and_then(|env, defs_or_expr| env.typing(&defs_or_expr))
            .get_pair();
        self = env;

        match res {
            Ok(TDefsOrExpr::Expression(expr)) => {
                let value = self.interpreter.eval_expr(&expr);
                println!("{value}")
            }
            Ok(TDefsOrExpr::Definitions(prog)) => {
                for def in prog.iter() {
                    self.interpreter = self.interpreter.add_definition(def.clone());
                    let name = def.get_name();
                    let ty = def.get_ty();
                    println!("{name} : {ty}");
                }
            }
            Err(err) => {
                eprintln!("{err}");
            }
        };
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
