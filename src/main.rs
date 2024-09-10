mod ast;
mod error;
mod location;
mod parser;
mod stdlib;
mod typing;
mod interpreter;

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
        .and_then(parser::parse_file)
        .inspect(|parse_tree| println!("sexp:\n{parse_tree}"))
        .and_then(parser::make_program)
        .inspect(|wt_program| println!("weak typed program:\n{wt_program}"))
        .and_then(typing::infer_type)
        .inspect(|t_program| println!("typed program:\n{t_program}"))
        .and_then(interpreter::eval_program)
        .inspect(|value| println!("value: {value}"));


    match res {
        Ok(value) => {
            std::process::exit(value.try_into().unwrap())
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1)
        }
    }
}
