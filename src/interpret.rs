use super::args::Args;
use super::{interpreter, parser, typing};
use color_print::cformat;

fn debug_print<T>(printing: bool, name: &str, elm: T)
where
    T: std::fmt::Display,
{
    if printing {
        let msg = cformat!("<cyan>{name} :</>\n{elm}");
        eprintln!("{msg}")
    }
}

pub fn interpret_file(args: &Args, file_name: &str) {
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
