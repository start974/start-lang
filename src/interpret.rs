use super::args::Args;
use super::utils::debug::{debug_sexp, debug_parser, debug_typer};
use super::utils::colored::Colored;
use super::{interpreter, parser, typing};


pub fn interpret_file(args: &Args, file_name: &str) {
    let res = Ok(file_name)
        .and_then(parser::parse_file)
        .inspect(|parse_tree| debug_sexp(args, &parse_tree))
        .and_then(parser::make_program)
        .inspect(|wt_program| debug_parser(args, wt_program))
        .and_then(typing::infer_type)
        .inspect(|t_program| debug_typer(args, t_program))
        .and_then(interpreter::eval_program);
    match res {
        Ok(ret_code) => std::process::exit(ret_code.try_into().unwrap()),
        Err(err) => {
            err.colored_eprintln(args);
            std::process::exit(err.get_code())
        }
    }
}
