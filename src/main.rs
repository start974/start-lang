mod parser;
mod ast;

use std::env;

fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a file name");
        std::process::exit(1)
    } else {
        args[1].clone()
    }
}

fn main() {
    let file_name = get_file_name();
    let parse_tree = parser::parse_file(file_name);
    println!("{:?}",parse_tree.to_sexp());
}
