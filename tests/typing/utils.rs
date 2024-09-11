use super::super::utils;
use startlang::error::Error;
use startlang::parser::{make_program, parse_file};
use startlang::typing::{ast::TProgram, infer_type};

fn f(file: &str) -> Result<TProgram, Error> {
    parse_file(file).and_then(make_program).and_then(infer_type)
}

const PREFIX: &str = "typing";

pub fn test_error(suffix: &str) {
    utils::test_error(PREFIX, suffix, f)
}

pub fn test_out(suffix: &str) {
    utils::test_out(PREFIX, suffix, f)
}
