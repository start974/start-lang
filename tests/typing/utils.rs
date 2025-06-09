use crate::utils;
use startlang::error::Errors;
use startlang::parser::{make_program, parse_file};
use startlang::typing::infer_type;

fn f(file: &str) -> Result<String, Errors> {
    parse_file(file)
        .and_then(make_program)
        .and_then(|(_, wt_prog)| infer_type(wt_prog))
        .map(|(_, t_prog)| utils::make_string(&t_prog))
}

const PREFIX: &str = "typing";

pub fn test_error(suffix: &str) {
    utils::test_error(PREFIX, suffix, f)
}

pub fn test_out(suffix: &str) {
    utils::test_out(PREFIX, suffix, f)
}
