use crate::args::Args;
use crate::utils::colored::Colored;
use crate::utils::debug::{debug_parser, debug_sexp, debug_typer};
use crate::{interpreter, parser, typing};

pub fn interpret_file(args: &Args, file_name: &str) {
    let res = Ok(file_name)
        .and_then(parser::parse_file)
        .inspect(|parse_tree| debug_sexp(args, parse_tree))
        .and_then(parser::make_program)
        .inspect(|(parser, wt_program)| debug_parser(args, parser, wt_program))
        .and_then(|(_, wt_program)| typing::infer_type(wt_program))
        .inspect(|(typer, t_program)| debug_typer(args, typer, t_program))
        .and_then(|(_, t_program)| interpreter::eval_program(t_program));
    match res {
        Ok(ret_code) => std::process::exit(ret_code),
        Err(err) => {
            err.colored_eprintln(args);
            std::process::exit(err.get_code())
        }
    }
}
