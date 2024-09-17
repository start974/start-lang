use crate::args::Args;
use crate::interpreter::Interpreter;
use crate::parser::{ast::WTDefsOrExpr, ParseTree, Parser};
use crate::stdlib::{NAME_ENV, TYPE_ENV};
use crate::typing::{ast::TDefsOrExpr, Typer};
use crate::utils::debug::*;
use crate::utils::{colored::Colored, FResult};

use rustyline::DefaultEditor;

struct Env {
    parse_tree: ParseTree<'static>,
    parser: Parser<'static>,
    typer: Typer,
    interpreter: Interpreter,
}

const FILE_NAME: &str = "<<stdin>>";

type EnvResult<T> = FResult<Env, T>;

impl Env {
    fn new() -> Self {
        let language = tree_sitter_start::start_repl_language();
        let parse_tree = ParseTree::make(FILE_NAME).set_language(&language);
        let parser = Parser::make(FILE_NAME, NAME_ENV.clone());
        let typer = Typer::make(TYPE_ENV.clone());
        let interpreter = Interpreter::empty();
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
            .inspect(|parser, defs_or_expr| {
                debug_parser(args, parser);
                debug_parsed_defs_or_expr(args, defs_or_expr)
            })
            .get_pair();
        self.parser = parser;
        EnvResult::make(self, res)
    }

    fn typing(mut self, args: &Args, defs_or_expr: &WTDefsOrExpr) -> EnvResult<TDefsOrExpr> {
        let (typer, res) = self
            .typer
            .type_definitions_or_expression(defs_or_expr)
            .inspect(|typer, defs_or_expr| {
                debug_typer(args, typer);
                debug_typed_defs_or_expr(args, defs_or_expr)
            })
            .get_pair();
        self.typer = typer;
        EnvResult::make(self, res)
    }

    fn eval(mut self, args: &Args, input: String) -> Self {
        let (env, res) = self
            // parse program
            .parse(args, input)
            // type program
            .and_then(|env, defs_or_expr| env.typing(args, &defs_or_expr))
            .get_pair();
        self = env;

        match res {
            Ok(TDefsOrExpr::Expression(expr)) => {
                let value = self.interpreter.eval_expr(&expr);
                debug_interpreter(args, &self.interpreter);
                value.colored_println(args);
            }
            Ok(TDefsOrExpr::Definitions(prog)) => {
                let (interpreter, defs) = prog.iter().fold(
                    (self.interpreter, Vec::new()),
                    |(interpreter, mut defs), def| {
                        let (interpreter, def) = interpreter.add_definition(def);
                        defs.push(def);
                        (interpreter, defs)
                    },
                );
                debug_interpreter(args, &interpreter);
                self.interpreter = interpreter;
                for def in defs {
                    def.colored_println(args);
                }
            }
            Err(err) => {
                err.colored_println(args);
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
