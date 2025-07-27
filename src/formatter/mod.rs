use crate::interpreter::Interpreter as _;
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use colored::Colorize as _;
use error::ErrorFileWrite;
use similar::{ChangeTag, TextDiff};
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod error;
mod interpreter;
mod parsed_file;

pub enum Mode {
    Overwrite,
    Diff,
    Print,
}

pub fn diff(original: &str, formatted: &str) -> bool {
    let diff = TextDiff::from_lines(original, formatted);
    let mut has_diff = false;
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                print!("{}", format!("-{change}").red());
                has_diff = true;
            }
            ChangeTag::Insert => {
                print!("{}", format!("+{change}").green());
                has_diff = true;
            }
            ChangeTag::Equal => {
                print!(" {change}");
            }
        }
    }
    has_diff
}

fn write_file(path: &Path, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    write!(file, "{content}")
}

/// run formatter on file
pub fn run(path: &Path, mode: Mode) -> i32 {
    let mut interpreter = interpreter::Interpreter::new(path);
    interpreter.run();
    if let Some(parsed_file) = interpreter.parsed_file() {
        let theme = Theme::default();
        let str = parsed_file.to_string(&theme);
        match mode {
            Mode::Overwrite => {
                if write_file(path, &str).is_err() {
                    interpreter.fail(ErrorFileWrite::new(path.to_path_buf()))
                }
            }
            Mode::Diff => {
                if diff(interpreter.get_content(), &str) {
                    interpreter.set_error_code(1);
                }
            }
            Mode::Print => println!("{str}"),
        }
    }
    interpreter.get_error_code()
}
