mod ast;
mod error;
mod location;
mod parser;
mod typing;

fn get_file_name() -> Result<String, error::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let err = error::Error::error_simple("Please provide a file name");
        Err(err)
    } else {
        Ok(args[1].clone())
    }
}

fn main() {
    let res = get_file_name()
        .and_then(parser::ParseTree::of_file)
        .inspect(|parse_tree| println!("sexp: {}", parse_tree.to_sexp()))
        .and_then(|parse_tree| parse_tree.to_program());

    match res {
        Ok(program) => {
            println!("{program}");
            std::process::exit(0)
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1)
        }
    }
}
