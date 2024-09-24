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
use error::*;
use utils::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "filename")]
    pub file_name: Option<String>,

    #[arg(long, help = "Set theme file")]
    pub theme: Option<String>,

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

const ERROR_FILE_THEME_NOT_FOUND: i32 = 105;
fn set_theme(opt_file_name: Option<String>) {
    opt_file_name
        .map_or_else(
            || {
                Ok((
                    "<<default-theme>>".to_string(),
                    include_str!("../assets/theme.yml").to_string(),
                ))
            },
            |theme_file| {
                std::fs::read_to_string(&theme_file)
                    .map_err(|_| {
                        let msg = Head::new()
                            .text("Unable to read theme file")
                            .quoted(&theme_file);
                        let err = Error::make(msg, ERROR_FILE_THEME_NOT_FOUND);
                        Errors::from(err)
                    })
                    .map(|content| (theme_file, content))
            },
        )
        .and_then(|(file_name, content)| theme::set_theme_from_yml(&file_name, content))
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(err.get_code());
        });
}

fn main() {
    let args = Args::parse();

    set_theme(args.theme);

    if args.no_color {
        SHOULD_COLORIZE.set_override(false)
    }

    if args.debug_sexp {
        debug::DEBUG_SEXP.activate();
    }
    if args.debug_parser {
        debug::DEBUG_PARSER.activate();
    }
    if args.debug_typer {
        debug::DEBUG_TYPER.activate();
    }
    if args.debug_interpreter {
        debug::DEBUG_INTERPRETER.activate();
    }

    match &args.file_name {
        None => repl::repl(),
        Some(file_name) => interpret::interpret_file(file_name),
    }
}
