use crate::utils;
use startlang::error::Error;
use startlang::interpreter::eval_program;
use startlang::parser::{make_program, parse_file};
use startlang::typing::infer_type;

fn f(file: &str) -> Result<i32, Error> {
    parse_file(file)
        .and_then(make_program)
        .and_then(infer_type)
        .and_then(eval_program)
}

const PREFIX: &str = "interpreter";

pub fn test_error(suffix: &str) {
    utils::test_error(PREFIX, suffix, f)
}

pub fn test_ret(suffix: &str) {
    utils::test_ret(PREFIX, suffix, f)
}
