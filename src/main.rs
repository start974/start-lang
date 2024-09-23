mod ast;
mod error;
mod interpret;
mod interpreter;
mod location;
mod parser;
mod repl;
mod stdlib;
mod typing;
mod utils;

use clap::Parser;
use colored::control::SHOULD_COLORIZE;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "filename")]
    pub file_name: Option<String>,

    #[arg(long, help = "Print sexp tree-sitter parser")]
    pub debug_sexp: bool,

    #[arg(long, help = "Print the AST")]
    pub debug_parser: bool,

    #[arg(long, help = "Print the typed program")]
    pub debug_typer: bool,

    #[arg(long, help = "Print the interpreter")]
    pub debug_interpreter: bool,

    #[arg(long, help = "Don't colorize error")]
    pub no_color: bool,
}

fn main() {
    let args = Args::parse();
    if args.no_color {
        SHOULD_COLORIZE.set_override(false)
    }
    if args.debug_sexp {
        utils::debug::DEBUG_SEXP.activate();
    }
    if args.debug_parser {
        utils::debug::DEBUG_PARSER.activate();
    }
    if args.debug_typer {
        utils::debug::DEBUG_TYPER.activate();
    }
    if args.debug_interpreter {
        utils::debug::DEBUG_INTERPRETER.activate();
    }

    match &args.file_name {
        None => repl::repl(),
        Some(file_name) => interpret::interpret_file(file_name),
    }
}
