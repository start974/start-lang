use std::{env, process::exit};

mod interpreter;
mod parser;
mod stdlib;
mod typing;
mod utils;
mod vm;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    match args.len() {
        1 => interpreter::repl(),
        2 => {
            let path = args.get(1).unwrap();
            let code = interpreter::file(path);
            exit(code)
        }
        _ => {
            eprintln!("Usage: {} [file.st]", args[0]);
            std::process::exit(1)
        }
    }
}
