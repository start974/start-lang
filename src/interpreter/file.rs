use super::interpreter::Interpreter;

fn run_file(path: &str) -> Result<(), i32> {
    let mut interpreter = Interpreter::new();
    let source_id = interpreter.set_file(path)?;
    interpreter.run(source_id)
}
pub fn file(path: &str) -> i32 {
    run_file(path).map(|_| 0).unwrap_or_else(|code| code)
}
