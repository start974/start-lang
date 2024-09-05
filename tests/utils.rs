use startlang::error::Error;
use pretty_assertions::{assert_eq, assert_ne};

fn get_output(file_name: &String) -> String {
    let file = format!("{file_name}.out");
    std::fs::read_to_string(file).unwrap()
}

fn get_input<F, T>(file_name:&String, f: F) -> Result<T, Error>
where
    F: Fn(String) -> Result<T, Error>,
{
    let file = format!("{file_name}.st");
    f(file)
}

fn get_file_name(prefix: &str, suffix:&str) -> String{
    format!("tests/{prefix}/{suffix}")
}

pub fn test_error<F, T>(prefix: &str, suffix:&str, f: F)
where
    F: Fn(String) -> Result<T, Error>,
    T: std::fmt::Display,
{
    let file_name = get_file_name(prefix, suffix);
    match get_input(&file_name, f) {
        Err(e) => assert_eq!(format!("{e}"), get_output(&file_name)),
        Ok(p) => panic!("Expected error found:\n {p}"),
    }
}

pub fn test_ok<F, T>(prefix: &str, suffix: &str, f: F)
where
    F: Fn(String) -> Result<T, Error>,
    T: std::fmt::Display,
{
    let file_name = get_file_name(prefix, suffix);
    match get_input(&file_name, f) {
        Ok(p) => assert_eq!(format!("{p}"), get_output(&file_name)),
        Err(e) => panic!("Expected no error found \n {e}"),
    }
}
