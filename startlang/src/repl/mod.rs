//! REPL: each input (lines up to one ending with `.`) goes through every pass;
//! the definitions are kept between inputs.

pub mod summary;

use crate::compile::compile;
use crate::printer::Printer;
use driver::stdlib::Stdlib;
use ir::location::SourceId;
use rustyline::{DefaultEditor, Editor, error::ReadlineError, history::FileHistory};

const HISTORY_FILE: &str = ".start-history.txt";

/// state of a REPL session
struct Session {
    /// definitions of the stdlib and of the previous inputs
    state: Stdlib,
    printer: Printer,
    /// every input so far: the source of the errors
    all_content: String,
}

impl Session {
    fn new() -> Self {
        Self {
            state: Stdlib::load(),
            printer: Printer::new(SourceId::Repl).with_summary(true),
            all_content: String::new(),
        }
    }

    /// run one input, after the previous ones
    fn input(&mut self, input: &str) {
        let offset = self.all_content.len();
        self.all_content.push_str(input);
        self.printer.set_content(self.all_content.clone());
        self.printer.reset_code();
        match compile(&mut self.state, SourceId::Repl, input, offset) {
            Ok(compiled) => compiled
                .outputs()
                .for_each(|output| self.printer.output(&output)),
            Err(errs) => self.printer.errors(errs),
        }
    }
}

fn finish(rl: &mut Editor<(), FileHistory>) {
    println!("Exiting REPL...");
    if rl.save_history(HISTORY_FILE).is_err() {
        eprintln!("Failed to save history");
    }
}

/// make prompt string
fn prompt_string(line_num: usize, many_line: bool) -> String {
    let sep = if many_line { "∥" } else { "⎮" };
    format!("{line_num:>4} {sep} ")
}

/// run repl
pub fn run() {
    let mut rl = DefaultEditor::new().expect("cannot start the line editor");
    let _ = rl.load_history(HISTORY_FILE);
    let mut session = Session::new();
    let mut line_num = 1;
    let mut many_line = false;
    let mut lines = String::new();
    loop {
        let ps = prompt_string(line_num, many_line);
        match rl.readline(&ps) {
            Ok(line) => {
                lines.push_str(&line);
                lines.push('\n');
                let _ = rl.add_history_entry(&line);
                if line.ends_with('.') {
                    session.input(&lines);
                    lines.clear();
                    many_line = false;
                } else {
                    many_line = true;
                }
            }
            Err(ReadlineError::Eof) => {
                session.input(&lines);
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
