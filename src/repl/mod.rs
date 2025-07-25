pub mod interpreter;
mod summary;

use crate::interpreter::Interpreter;
use rustyline::{error::ReadlineError, history::FileHistory, DefaultEditor, Editor};

const HISTORY_FILE: &str = ".start-history.txt";
fn finish(rl: &mut Editor<(), FileHistory>) {
    println!("Exiting REPL...");
    if rl.save_history(HISTORY_FILE).is_err() {
        eprintln!("Failed to save history");
    }
}

/// make promp string
fn prompt_string(line_num: usize, many_line: bool) -> String {
    let line_num_str = line_num.to_string();
    const MAX_LINE_NUM_LEN: usize = 3;
    let mut res = if line_num_str.len() <= MAX_LINE_NUM_LEN {
        " ".repeat(MAX_LINE_NUM_LEN - line_num_str.len())
    } else {
        String::new()
    };
    res.push_str(&line_num_str);
    if many_line {
        res.push_str(" ⎮ ");
    } else {
        res.push_str(" ∥ ");
    }
    res
}

/// run repl
pub fn run() {
    let mut rl = DefaultEditor::new().unwrap();
    let _ = rl.load_history(HISTORY_FILE);
    let mut interpreter = interpreter::Interpreter::default();
    let mut line_num = 1;
    let mut many_line = false;
    let mut lines = String::new();
    loop {
        let ps = prompt_string(line_num, many_line);
        match rl.readline(&ps) {
            Ok(line) => {
                lines.push_str(&line);
                lines.push('\n');
                rl.add_history_entry(&line).unwrap();
                if line.ends_with(".") {
                    interpreter.add_content(&lines);
                    interpreter.run();
                    lines = String::new();
                    many_line = false;
                } else {
                    many_line = true;
                }
            }
            Err(ReadlineError::Eof) => {
                interpreter.add_content(&lines);
                interpreter.run();
                finish(&mut rl);
                return;
            }

            Err(_) => {
                finish(&mut rl);
                return;
            }
        }
        line_num += 1;
    }
}
