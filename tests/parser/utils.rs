use startlang::error::Error;
use startlang::parser::{make_program, parse_file};

fn get_output(prefix: String) -> String {
    let output_file = prefix.to_string() + ".out";
    std::fs::read_to_string(output_file).unwrap()
}

pub fn test_parse_error(prefix: &str) {
    let prefix = "tests/parser/".to_string() + prefix;
    let input = prefix.clone() + ".st";
    let res_program = parse_file(input).and_then(make_program);
    match res_program {
        Err(e) => assert_eq!(format!("{e}"), get_output(prefix)),
        Ok(p) => panic!("Expected error found:\n {p}"),
    }
}

pub fn test_parse_ok(prefix: &str) {
    let prefix = "tests/parser/".to_string() + prefix;
    let input = prefix.clone() + ".st";
    let res_program = parse_file(input).and_then(make_program);
    match res_program {
        Ok(p) => assert_eq!(format!("{p}"), get_output(prefix)),
        Err(e) => panic!("Expected no error found \n {e}"),
    }
}
