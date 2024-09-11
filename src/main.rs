mod ast;
mod error;
mod interpreter;
mod location;
mod parser;
mod stdlib;
mod typing;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "filename")]
    file_name: Option<String>,

    #[arg(long, help = "Print sexp tree-sitter parser")]
    debug_sexp: bool,

    #[arg(long, help = "Print the AST")]
    debug_parser: bool,

    #[arg(long, help = "Print the typed program")]
    debug_typer: bool,

    #[arg(long, help = "Don't colorize error")]
    no_color: bool,
}

fn get_file_name() -> Result<String, error::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        let err = error::Error::error_simple("Please provide a file name");
        Err(err)
    } else {
        Ok(args[1].clone())
    }
}

fn debug_print(printing: bool, msg: String) {
    if printing {
        eprintln!("{msg}");
    }
}

fn eval_file(args: &Args, file_name: String) {
    let res = Ok(file_name)
        .and_then(parser::parse_file)
        .inspect(|parse_tree| debug_print(args.debug_sexp, format!("SEXP:\n{parse_tree}")))
        .and_then(parser::make_program)
        .inspect(|wt_program| {
            debug_print(args.debug_parser, format!("Parsed program:\n{wt_program}"))
        })
        .and_then(typing::infer_type)
        .inspect(|t_program| debug_print(args.debug_typer, format!("Typed program:\n{t_program}")))
        .and_then(interpreter::eval_program);
    match res {
        Ok(ret_code) => std::process::exit(ret_code.try_into().unwrap()),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1)
        }
    }
}

fn main() {
    let args = Args::parse();

    match &args.file_name {
        None => std::process::exit(0),
        Some(file_name) => eval_file(&args, file_name.to_string()),
    }
}
