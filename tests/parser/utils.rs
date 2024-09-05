use startlang::error::Error;
use startlang::parser::{ast::WTProgram, make_program, parse_file};
use super::super::utils;

fn f(file: String) -> Result<WTProgram, Error>{
    parse_file(file).and_then(make_program)
}

const PREFIX: &str = "parser";

pub fn test_error(suffix: &str) {
    utils::test_error(PREFIX, suffix, f)
}

pub fn test_ok(suffix: &str) {
    utils::test_ok(PREFIX, suffix, f)
}

