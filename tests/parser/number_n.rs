use startlang::error::Error;
use startlang::parser::{ast::Program, ParseTree};

fn test_parse(input: &str) -> Result<Program, Error> {
    ParseTree::of_string("test".to_string(), &input.to_string()).to_program()
}

#[test]
fn test_parse_simple() {
    let program = test_parse("def a := 2").unwrap();
    assert_eq!(format!("{program}"), "def a := 2\n");
}

#[test]
fn test_parse_simple_typed() {
    let program = test_parse("def a : N := 2").unwrap();
    let mut it = program.iter();
    let def = it.next().unwrap();
    assert_eq!(format!("{def}"), "def a : N := 2");
}

#[test]
fn test_parse_two() {
    let program = test_parse("def a := 2\ndef b := 1").unwrap();
    let mut it = program.iter();
    let mut def = it.next().unwrap();
    assert_eq!(format!("{def}"), "def a := 2");
    def = it.next().unwrap();
    assert_eq!(format!("{def}"), "def b := 1");
}

#[test]
fn test_parse_two_ty() {
    let program = test_parse("def a : N := 2  \ndef   b  : N:= 1").unwrap();
    let mut it = program.iter();

    let mut def = it.next().unwrap();
    assert_eq!(format!("{def}"), "def a : N := 2");

    def = it.next().unwrap();
    assert_eq!(format!("{def}"), "def b : N := 1");
}

#[test]
fn test_parse_two_one_ty() {
    let program = test_parse("def a : N := 2 def b := 1").unwrap();
    let mut it = program.iter();

    let mut def = it.next().unwrap();
    assert_eq!(format!("{def}"), "def a : N := 2");

    def = it.next().unwrap();
    assert_eq!(format!("{def}"), "def b := 1");
}

#[test]
fn test_parse_same_def() {
    let program = test_parse("def a : N := 2 def a := 1").unwrap();
    let mut it = program.iter();

    let mut def = it.next().unwrap();
    assert_eq!(format!("{def}"), "def a : N := 2");

    def = it.next().unwrap();
    assert_eq!(format!("{def}"), "def a := 1");
}
