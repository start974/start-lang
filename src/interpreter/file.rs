use super::interpret::Interpreter;

pub fn file(path: &str) -> i32 {
    let mut interpreter = Interpreter::new();
    match interpreter.set_file(path) {
        Ok(source_id) => {
            interpreter.run(source_id);
            interpreter.get_err_code()
        }
        Err(code) => code,
    }
}
