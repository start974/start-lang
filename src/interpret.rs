use crate::args::Args;
use crate::utils::colored::Colored;
use crate::utils::debug::*;
use crate::{interpreter, parser, typing};

pub fn interpret_file(file_name: &str) {
    let res = Ok(file_name)
        // parse tree
        .and_then(parser::parse_file)
        .inspect(|parse_tree| debug_sexp(parse_tree))
        // parser program
        .and_then(parser::make_program)
        .inspect(|(parser, prog)| {
            debug_parser(parser);
            debug_parsed_prog(prog)
        })
        // type program
        .and_then(|(_, prog)| typing::infer_type(prog))
        .inspect(|(typer, prog)| {
            debug_typer(typer);
            debug_typed_prog(prog)
        })
        // interpret program
        .and_then(|(_, prog)| interpreter::eval_program(prog))
        .inspect(|(interpreter, ret)| {
            debug_interpreter(interpreter);
            debug_i32_res(ret)
        })
        // map result
        .map(|(_, ret)| ret);
    match res {
        Ok(ret_code) => std::process::exit(ret_code),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(err.get_code())
        }
    }
}
