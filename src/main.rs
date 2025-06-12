use std::{env, process::exit};

mod interpreter;
mod parser;
mod repl;
mod stdlib;
mod typing;
mod utils;
mod vm;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    } else if args.len() == 2 {
        let path = args.get(1).unwrap();
        let code = interpreter::file(path);
        exit(code);
    } else {
        interpreter::repl()
    }
}
