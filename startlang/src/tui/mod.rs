//! `startlang tui`: the interactive session in a terminal UI.

mod app;
mod editor;
mod output;
mod session;
mod terminal;

use crate::error;
use crate::printer::Printer;
use app::App;
use editor::History;
use ir::location::SourceId;
use std::io::{self, IsTerminal};
use std::path::Path;
use terminal::Terminal;

/// the inputs of the previous sessions, in the working directory
const HISTORY_FILE: &str = ".start-history.txt";

/// run the TUI until the user quits; returns the exit code
pub fn run() -> i32 {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        let mut printer = Printer::new(SourceId::Unknown);
        printer.fail(&error::no_terminal());
        return printer.code();
    }
    match run_in(Path::new(HISTORY_FILE)) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("startlang tui: {err}");
            1
        }
    }
}

/// the TUI in the terminal, with the history in `history`
fn run_in(history: &Path) -> io::Result<()> {
    let mut app = App::new(History::load(history));
    let mut terminal = Terminal::init()?;
    while !app.quit() {
        terminal.inner.draw(|frame| app.draw(frame))?;
        app.event(crossterm::event::read()?);
    }
    // the terminal back before writing anything
    drop(terminal);
    app.history().save(history)
}
