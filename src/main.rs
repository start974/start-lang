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
mod utils;

use clap::Parser;

fn main() {
    let args = args::Args::parse();

    match &args.file_name {
        None => repl::repl(&args),
        Some(file_name) => interpret::interpret_file(&args, file_name),
    }
}
