use crate::ast::pretty_print::*;
use crate::error::Errors;
use crate::interpreter::{DefsOrValue, Interpreter};
use crate::parser::{ast::WTDefsOrExpr, ParseTree, Parser};
use crate::stdlib::{NAME_ENV, TYPE_ENV};
use crate::typing::{ast::TDefsOrExpr, Typer};
use crate::utils::debug::*;
use crate::utils::FResult;
use rustyline::{history::FileHistory, DefaultEditor, Editor};

struct Env {
    parse_tree: ParseTree<'static>,
    parser: Parser<'static>,
    typer: Typer,
    interpreter: Interpreter,
}

const FILE_NAME: &str = "<<stdin>>";

type EnvResult<T> = FResult<Env, T, Errors>;

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

    fn parse(mut self, input: String) -> EnvResult<WTDefsOrExpr> {
        // make a part tree
        self.parse_tree = self.parse_tree.set_content(input).parse();
        debug_sexp(&self.parse_tree);

        // parse
        let content = self.parse_tree.get_content();
        let node = self.parse_tree.root_node();
        let (parser, res) = self
            .parser
            .set_content(&content)
            .parse_definitions_or_expression(&node)
            .inspect(|parser, defs_or_expr| {
                debug_parser(parser);
                debug_parsed_defs_or_expr(defs_or_expr)
            })
            .get_pair();
        self.parser = parser;
        EnvResult::make(self, res)
    }

    fn typing(mut self, defs_or_expr: &WTDefsOrExpr) -> EnvResult<TDefsOrExpr> {
        let (typer, res) = self
            .typer
            .type_definitions_or_expression(defs_or_expr)
            .inspect(|typer, defs_or_expr| {
                debug_typer(typer);
                debug_typed_defs_or_expr(defs_or_expr)
            })
            .get_pair();
        self.typer = typer;
        EnvResult::make(self, res)
    }

    fn interpret(mut self, defs_or_expr: &TDefsOrExpr) -> EnvResult<DefsOrValue> {
        let (interpreter, defs_or_val) = self
            .interpreter
            .eval_definitions_or_expression(defs_or_expr);
        debug_interpreter(&interpreter);
        self.interpreter = interpreter;
        EnvResult::ok(self, defs_or_val)
    }

    fn eval(self, input: &String) -> Self {
        let (env, res) = self
            // parse
            .parse(input.to_owned())
            // type
            .and_then(|env, defs_or_expr| env.typing(&defs_or_expr))
            // interpret
            .and_then(|env, defs_or_expr| env.interpret(&defs_or_expr))
            // get result
            .get_pair();

        match res {
            Ok(def_or_vals) => println!("{}", def_or_vals.to_string_colored()),
            Err(err) => eprintln!("{}", err),
        };
        env
    }
}

const HISTORY_FILE: &str = ".start-history.txt";
fn finish(rl: &mut Editor<(), FileHistory>) {
    if rl.save_history(HISTORY_FILE).is_err() {
        eprintln!("Failed to save history");
    }
}
pub fn repl() {
    let mut env = Env::new();
    let mut rl = DefaultEditor::new().unwrap();
    let _ = rl.load_history(HISTORY_FILE);
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
                        Err(_) => {
                            finish(&mut rl);
                            return;
                        }
                    }
                }
                // interpret
                env = env.eval(&line);
                let _ = rl.add_history_entry(line);
            }
            Err(_) => {
                finish(&mut rl);
                return;
            }
        }
    }
}
