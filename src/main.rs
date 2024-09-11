mod ast;
mod error;
mod interpreter;
mod location;
mod parser;
mod stdlib;
mod typing;

use clap::Parser;
use color_print::cformat;

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

fn debug_print<T>(printing: bool, name: &str, elm: T)
where
    T: std::fmt::Display,
{
    if printing {
        let msg = cformat!("<cyan>{name} :</>\n{elm}");
        eprintln!("{msg}")
    }
}

fn eval_file(args: &Args, file_name: String) {
    let res = Ok(file_name)
        .and_then(parser::parse_file)
        .inspect(|parse_tree| debug_print(args.debug_sexp, "SEXP", parse_tree))
        .and_then(parser::make_program)
        .inspect(|wt_program| debug_print(args.debug_parser, "Parsed program", wt_program))
        .and_then(typing::infer_type)
        .inspect(|t_program| debug_print(args.debug_typer, "Typed program", t_program))
        .and_then(interpreter::eval_program);
    match res {
        Ok(ret_code) => std::process::exit(ret_code.try_into().unwrap()),
        Err(err) => {
            let msg = if args.no_color {
                err.to_string()
            } else {
                err.colored()
            };
            eprint!("{msg}");
            std::process::exit(err.get_code())
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
