use super::interpreter::Interpreter;
use rustyline::{history::FileHistory, DefaultEditor, Editor};

const HISTORY_FILE: &str = ".start-history.txt";
fn finish(rl: &mut Editor<(), FileHistory>) {
    if rl.save_history(HISTORY_FILE).is_err() {
        eprintln!("Failed to save history");
    }
}

/// run code
pub fn run(interpreter: &mut Interpreter) {
    let source_id = interpreter.last_repl_source_id();
    interpreter.run(source_id);
    interpreter.update_repl();
}

/// make promp string
pub fn prompt_string(line_num: usize, many_line: bool) -> String {
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

pub fn repl() {
    let mut rl = DefaultEditor::new().unwrap();
    let _ = rl.load_history(HISTORY_FILE);
    let mut interpreter = Interpreter::new();
    interpreter.set_repl_mod(true);
    let mut line_num = 1;
    let mut many_line = false;
    loop {
        let ps = prompt_string(line_num, many_line);
        match rl.readline(&ps) {
            Ok(line) => {
                interpreter.add_repl(&line);
                if line.ends_with(".") {
                    run(&mut interpreter);
                    many_line = false;
                } else {
                    many_line = true;
                }
            }
            Err(_) => {
                finish(&mut rl);
                return;
            }
        }
        line_num += 1;
    }
}
