use error::ErrorFileWrite;

use crate::interpreter::Interpreter as _;
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod error;
mod interpreter;
mod parsed_file;

/// run formatter on file
pub fn run(path: &Path, print: bool) -> i32 {
    let mut interpreter = interpreter::Interpreter::new(path);
    interpreter.run();
    match interpreter.parsed_file() {
        None => (),
        Some(parsed_file) => {
            let theme = Theme::default();
            let str = parsed_file.to_string(&theme);
            if print {
                println!("{}", str);
            } else {
                let mut file = File::open(path).unwrap();
                file.write_all(str.as_bytes()).unwrap_or_else(|_| {
                    let err = ErrorFileWrite::new(path.to_path_buf());
                    interpreter.fail(err);
                });
            }
        }
    }
    interpreter.get_error_code()
}
