use std::path::Path;

mod diff;
mod error;
mod formatter;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Overwrite,
    Diff,
    Print,
}

/// run formatter on file
pub fn run(path: &Path, mode: Mode) -> i32 {
    let mut formatter = formatter::Formatter::new(path);
    formatter.run(&mode);
    formatter.err_code
}
