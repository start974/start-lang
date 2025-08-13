use crate::interpreter::Interpreter as _;
use std::path::Path;

pub mod error;
pub mod interpreter;

pub use interpreter::Interpreter;

/// intrerpet file
pub fn run(path: &Path) -> i32 {
    let mut interpreter = interpreter::Interpreter::new(path);
    interpreter.run();
    interpreter.get_error_code()
}
