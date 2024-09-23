use crate::utils;
use startlang::error::Errors;
use startlang::interpreter::eval_program;
use startlang::parser::{make_program, parse_file};
use startlang::typing::{check_main, infer_type};

fn f(file: &str) -> Result<i32, Errors> {
    parse_file(file)
        .and_then(make_program)
        .and_then(|(_, wt_prog)| infer_type(wt_prog))
        .and_then(|(typer, t_prog)| check_main(&typer).map(|()| t_prog))
        .and_then(eval_program)
        .map(|(_, res)| res)
}

const PREFIX: &str = "interpreter";

pub fn test_error(suffix: &str) {
    utils::test_error(PREFIX, suffix, f)
}

pub fn test_ret(suffix: &str) {
    utils::test_ret(PREFIX, suffix, f)
}
