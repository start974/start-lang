mod args;
mod ast;
mod error;
mod interpret;
mod interpreter;
mod location;
mod parser;
mod repl;
mod stdlib;
mod typing;

use clap::Parser;

fn main() {
    let args = args::Args::parse();

    match &args.file_name {
        None => repl::repl(),
        Some(file_name) => interpret::interpret_file(&args, file_name.to_string()),
    }
}
