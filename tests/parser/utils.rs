use crate::utils;
use startlang::error::Errors;
use startlang::parser::{make_program, parse_file};

fn f(file: &str) -> Result<String, Errors> {
    parse_file(file)
        .and_then(make_program)
        .map(|(_, wt_prog)| utils::make_string(&wt_prog))
}

const PREFIX: &str = "parser";

pub fn test_error(suffix: &str) {
    utils::test_error(PREFIX, suffix, f)
}

pub fn test_out(suffix: &str) {
    utils::test_out(PREFIX, suffix, f)
}
