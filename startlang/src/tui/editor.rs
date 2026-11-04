//! The input area of `startlang tui`, without terminal: a text area edited
//! over several lines, the keys of the TUI, and the history of the inputs.
//! The keys are those of a classic shell (readline):
//!
//! | key                     | action                                       |
//! |-------------------------|----------------------------------------------|
//! | Enter                   | run the whole input                          |
//! | Shift+Enter, Alt+Enter  | new line (Alt+Enter in every terminal)       |
//! | ↑ / Ctrl+P              | previous input (↑ on the first line)         |
//! | ↓ / Ctrl+N              | next input (↓ on the last line)              |
//! | Ctrl+A / Ctrl+E         | start / end of the line                      |
//! | Ctrl+U / Ctrl+K         | delete to the start / end of the line        |
//! | Ctrl+W                  | delete the word before                       |
//! | Alt+← / Alt+→ (or B / F)| word back / forward (Ctrl+← / Ctrl+→ too)    |
//! | Ctrl+Y                  | paste what was deleted                       |
//! | Ctrl+C                  | clear the input; quit on an empty one        |
//! | Ctrl+D                  | quit on an empty input                       |
//! | Ctrl+L                  | clear the outputs                            |
//! | PageUp / PageDown, wheel| scroll the outputs, in the TUI               |
//! | F1, Alt+H               | help, in the TUI (the same key or Esc closes) |
//! | others                  | edit the text (`ratatui_textarea`)           |

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::style::Style;
use ratatui_textarea::{CursorMove, TextArea};
use std::io;
use std::path::Path;

/// the keys of the TUI and what they do, for the help
pub const KEYS: &[(&str, &str)] = &[
    ("Enter", "run the whole input"),
    ("Shift+Enter, Alt+Enter", "new line"),
    ("↑ / Ctrl+P", "previous input (↑ on the first line)"),
    ("↓ / Ctrl+N", "next input (↓ on the last line)"),
    ("Ctrl+A / Ctrl+E", "start / end of the line"),
    ("Ctrl+U / Ctrl+K", "delete to the start / end of the line"),
    ("Ctrl+W", "delete the word before"),
    ("Alt+← / Alt+→", "word back / forward (also Alt+B / Alt+F)"),
    ("Ctrl+Y", "paste what was deleted"),
    ("Ctrl+C", "clear the input; quit on an empty one"),
    ("Ctrl+D", "quit on an empty input"),
    ("Ctrl+L", "clear the outputs"),
    ("PageUp / PageDown, wheel", "scroll the outputs"),
    ("F1, Alt+H", "this help (the same key or Esc to close)"),
];

// ============================================================================
// Action
// ============================================================================

/// what a key asks the TUI to do, besides editing the input
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    /// nothing more: the input was edited, or the key does nothing
    None,
    /// run this input (the input area is now empty)
    Run(String),
    Quit,
    ClearOutputs,
}

// ============================================================================
// Editor
// ============================================================================

/// the input being typed, and the previous inputs
pub struct Editor {
    textarea: TextArea<'static>,
    history: History,
}

/// an empty input, after the previous inputs `history`
impl From<History> for Editor {
    fn from(history: History) -> Self {
        Self {
            textarea: new_textarea(vec![String::new()]),
            history,
        }
    }
}

impl Editor {
    /// the input area, to draw it
    pub fn textarea(&self) -> &TextArea<'static> {
        &self.textarea
    }

    /// the previous inputs, to save them
    pub fn history(&self) -> &History {
        &self.history
    }

    /// the input, its lines joined
    pub fn input(&self) -> String {
        self.textarea.lines().join("\n")
    }

    /// handle a key: edit the input, or ask the TUI for an action
    pub fn key(&mut self, key: KeyEvent) -> Action {
        // with the keyboard enhancement, keys are also reported when released
        if key.kind == KeyEventKind::Release {
            return Action::None;
        }
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let alt = key.modifiers.contains(KeyModifiers::ALT);
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);
        let (row, last) = (self.textarea.cursor().0, self.textarea.lines().len() - 1);
        // the keys of the text area are those of readline (Ctrl+A, Ctrl+E,
        // Ctrl+K, Ctrl+W, Ctrl+Y, Alt+B, Alt+F), but for these
        match key.code {
            // Shift+Enter only if the terminal reports it (keyboard
            // enhancement); Alt+Enter everywhere
            KeyCode::Enter if shift || alt => self.textarea.insert_newline(),
            KeyCode::Enter => return self.run(),
            // words: Alt+B / Alt+F in the text area, not the arrows
            KeyCode::Left if alt || ctrl => self.textarea.move_cursor(CursorMove::WordBack),
            KeyCode::Right if alt || ctrl => self.textarea.move_cursor(CursorMove::WordForward),
            KeyCode::Up if ctrl || row == 0 => self.previous(),
            KeyCode::Down if ctrl || row == last => self.next(),
            // history, not cursor moves as in the text area
            KeyCode::Char('p') if ctrl => self.previous(),
            KeyCode::Char('n') if ctrl => self.next(),
            // delete to the start of the line, not "undo" as in the text area
            KeyCode::Char('u') if ctrl => {
                self.textarea.delete_line_by_head();
            }
            KeyCode::Char('c') if ctrl => {
                if self.textarea.is_empty() {
                    return Action::Quit;
                }
                self.set_input("", CursorMove::Bottom);
                self.history.reset();
            }
            KeyCode::Char('d') if ctrl && self.textarea.is_empty() => return Action::Quit,
            KeyCode::Char('l') if ctrl => return Action::ClearOutputs,
            _ => {
                self.textarea.input(key);
            }
        }
        Action::None
    }

    /// pasted text: inserted with its line breaks, not run
    pub fn paste(&mut self, text: &str) {
        self.textarea
            .insert_str(text.replace("\r\n", "\n").replace('\r', "\n"));
    }

    /// run the input, if it is not blank: it goes into the history
    fn run(&mut self) -> Action {
        let input = self.input();
        if input.trim().is_empty() {
            return Action::None;
        }
        self.history.push(input.clone());
        self.set_input("", CursorMove::Bottom);
        Action::Run(input)
    }

    /// the previous input, the cursor on its first line (↑ again goes on)
    fn previous(&mut self) {
        let current = self.input();
        if let Some(input) = self.history.previous(&current).map(str::to_string) {
            self.set_input(&input, CursorMove::Top);
        }
    }

    /// the next input, the cursor on its last line (↓ again goes on)
    fn next(&mut self) {
        if let Some(input) = self.history.next().map(str::to_string) {
            self.set_input(&input, CursorMove::Bottom);
        }
    }

    /// replace the input; the cursor at the end of the line `line` goes to
    fn set_input(&mut self, input: &str, line: CursorMove) {
        self.textarea = new_textarea(input.split('\n').map(str::to_string).collect());
        self.textarea.move_cursor(line);
        self.textarea.move_cursor(CursorMove::End);
    }
}

/// a text area with `lines`, no underlined cursor line (its line numbers are
/// drawn by the TUI, with a separator)
fn new_textarea(lines: Vec<String>) -> TextArea<'static> {
    let mut textarea = TextArea::new(lines);
    textarea.set_cursor_line_style(Style::new());
    textarea
}

// ============================================================================
// History
// ============================================================================

/// the inputs run so far, oldest first, and where the user is in them; a
/// recalled input is a copy: editing it does not change the history
#[derive(Debug, Default, PartialEq, Eq)]
pub struct History {
    inputs: Vec<String>,
    /// the input recalled, `None` while typing a new one
    position: Option<usize>,
    /// the input being typed when the history was entered, back after the
    /// newest input
    draft: String,
}

/// the inputs, oldest first
impl From<Vec<String>> for History {
    fn from(inputs: Vec<String>) -> Self {
        Self {
            inputs,
            ..Self::default()
        }
    }
}

impl History {
    /// an input just run; the same one twice in a row is kept once
    pub fn push(&mut self, input: String) {
        if self.inputs.last() != Some(&input) {
            self.inputs.push(input);
        }
        self.reset();
    }

    /// back to typing a new input
    pub fn reset(&mut self) {
        self.position = None;
    }

    /// the input before the one recalled; `current` is kept as the draft
    /// when the history is entered; `None` at the oldest one
    pub fn previous(&mut self, current: &str) -> Option<&str> {
        let position = match self.position {
            None if self.inputs.is_empty() => return None,
            None => {
                self.draft = current.to_string();
                self.inputs.len() - 1
            }
            Some(0) => return None,
            Some(position) => position - 1,
        };
        self.position = Some(position);
        Some(&self.inputs[position])
    }

    /// the input after the one recalled, or the draft after the newest one;
    /// `None` while typing a new input
    pub fn next(&mut self) -> Option<&str> {
        let position = self.position?;
        if position + 1 < self.inputs.len() {
            self.position = Some(position + 1);
            Some(&self.inputs[position + 1])
        } else {
            self.position = None;
            Some(&self.draft)
        }
    }

    /// the history saved in `path`; empty if there is none
    pub fn load(path: &Path) -> Self {
        let text = std::fs::read_to_string(path).unwrap_or_default();
        Self::from(text.lines().map(decode).collect::<Vec<_>>())
    }

    /// save the history in `path`, one input per line
    pub fn save(&self, path: &Path) -> io::Result<()> {
        let lines: Vec<_> = self.inputs.iter().map(|input| encode(input)).collect();
        std::fs::write(path, lines.join("\n") + "\n")
    }
}

/// an input on one line: `\` and line breaks escaped
fn encode(input: &str) -> String {
    input.replace('\\', "\\\\").replace('\n', "\\n")
}

/// the input of a line written by `encode`
fn decode(line: &str) -> String {
    let mut input = String::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match (c, c == '\\') {
            (_, true) => match chars.next() {
                Some('n') => input.push('\n'),
                Some(other) => input.push(other),
                None => input.push('\\'),
            },
            (c, false) => input.push(c),
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, modifiers)
    }

    fn plain(code: KeyCode) -> KeyEvent {
        key(code, KeyModifiers::NONE)
    }

    /// type `text`, a key per character
    fn type_text(editor: &mut Editor, text: &str) {
        for c in text.chars() {
            assert_eq!(editor.key(plain(KeyCode::Char(c))), Action::None);
        }
    }

    fn editor() -> Editor {
        Editor::from(History::default())
    }

    #[test]
    fn enter_runs_the_whole_input() {
        let mut editor = editor();
        type_text(&mut editor, "Def a := 1.");
        editor.key(key(KeyCode::Enter, KeyModifiers::SHIFT));
        type_text(&mut editor, "Eval a.");
        assert_eq!(
            editor.key(plain(KeyCode::Enter)),
            Action::Run("Def a := 1.\nEval a.".to_string())
        );
        assert_eq!(editor.input(), "");
        // a blank input does not run
        type_text(&mut editor, "  ");
        assert_eq!(editor.key(plain(KeyCode::Enter)), Action::None);
    }

    #[test]
    fn alt_enter_is_a_new_line_too() {
        let mut editor = editor();
        type_text(&mut editor, "Def a := 1.");
        editor.key(key(KeyCode::Enter, KeyModifiers::ALT));
        type_text(&mut editor, "Eval a.");
        assert_eq!(editor.input(), "Def a := 1.\nEval a.");
    }

    #[test]
    fn keys_of_a_shell() {
        let mut editor = Editor::from(History::from(vec!["Eval 1.".to_string()]));
        let ctrl = |c| key(KeyCode::Char(c), KeyModifiers::CONTROL);
        type_text(&mut editor, "Def a := 2.");
        // Ctrl+U deletes to the start of the line, Ctrl+Y pastes it back
        editor.key(ctrl('u'));
        assert_eq!(editor.input(), "");
        editor.key(ctrl('y'));
        assert_eq!(editor.input(), "Def a := 2.");
        // Ctrl+A / Ctrl+E: start / end of the line
        editor.key(ctrl('a'));
        assert_eq!(editor.textarea().cursor().1, 0);
        editor.key(ctrl('e'));
        assert_eq!(editor.textarea().cursor().1, 11);
        // Alt+← / Alt+→: a word back / forward (the `.` is a word of its own)
        editor.key(key(KeyCode::Left, KeyModifiers::ALT));
        assert_eq!(editor.textarea().cursor().1, 10);
        editor.key(key(KeyCode::Right, KeyModifiers::ALT));
        assert_eq!(editor.textarea().cursor().1, 11);
        // Ctrl+W deletes the word before (here the `.`: punctuation is a word
        // of its own in the text area)
        editor.key(ctrl('w'));
        assert_eq!(editor.input(), "Def a := 2");
        // Ctrl+P / Ctrl+N: the history
        editor.key(ctrl('p'));
        assert_eq!(editor.input(), "Eval 1.");
        editor.key(ctrl('n'));
        assert_eq!(editor.input(), "Def a := 2");
    }

    #[test]
    fn released_keys_ignored() {
        let mut editor = editor();
        let mut release = plain(KeyCode::Char('x'));
        release.kind = KeyEventKind::Release;
        assert_eq!(editor.key(release), Action::None);
        assert_eq!(editor.input(), "");
    }

    #[test]
    fn up_and_down_through_the_history() {
        let mut editor = Editor::from(History::from(vec![
            "Eval 1.".to_string(),
            "Def a := 2.\nEval a.".to_string(),
        ]));
        type_text(&mut editor, "Eval 3");
        // the newest input, whole, the cursor on its first line
        editor.key(plain(KeyCode::Up));
        assert_eq!(editor.input(), "Def a := 2.\nEval a.");
        assert_eq!(editor.textarea().cursor().0, 0);
        editor.key(plain(KeyCode::Up));
        assert_eq!(editor.input(), "Eval 1.");
        // the oldest: ↑ does nothing more
        editor.key(plain(KeyCode::Up));
        assert_eq!(editor.input(), "Eval 1.");
        // ↓: back to the newest, the cursor on its last line, then the draft
        editor.key(plain(KeyCode::Down));
        assert_eq!(editor.input(), "Def a := 2.\nEval a.");
        assert_eq!(editor.textarea().cursor().0, 1);
        editor.key(plain(KeyCode::Down));
        assert_eq!(editor.input(), "Eval 3");
    }

    #[test]
    fn arrows_move_the_cursor_inside_an_input() {
        let mut editor = editor();
        type_text(&mut editor, "Def a := 1.");
        editor.key(key(KeyCode::Enter, KeyModifiers::SHIFT));
        type_text(&mut editor, "Eval a.");
        editor.key(plain(KeyCode::Enter));
        type_text(&mut editor, "x");
        editor.key(key(KeyCode::Enter, KeyModifiers::SHIFT));
        type_text(&mut editor, "y");
        // on the second line: ↑ moves to the first one
        editor.key(plain(KeyCode::Up));
        assert_eq!(editor.input(), "x\ny");
        assert_eq!(editor.textarea().cursor().0, 0);
        // Ctrl+↑ recalls from anywhere
        editor.key(plain(KeyCode::Down));
        editor.key(key(KeyCode::Up, KeyModifiers::CONTROL));
        assert_eq!(editor.input(), "Def a := 1.\nEval a.");
    }

    #[test]
    fn a_recalled_input_is_a_copy() {
        let mut editor = Editor::from(History::from(vec!["Eval 1.".to_string()]));
        editor.key(plain(KeyCode::Up));
        editor.key(plain(KeyCode::Backspace));
        type_text(&mut editor, " 2.");
        assert_eq!(
            editor.key(plain(KeyCode::Enter)),
            Action::Run("Eval 1 2.".to_string())
        );
        // the old input is still there, the new one after it
        assert_eq!(
            editor.history(),
            &History::from(vec!["Eval 1.".to_string(), "Eval 1 2.".to_string()])
        );
        editor.key(plain(KeyCode::Up));
        assert_eq!(editor.input(), "Eval 1 2.");
        editor.key(plain(KeyCode::Up));
        assert_eq!(editor.input(), "Eval 1.");
    }

    #[test]
    fn ctrl_c_ctrl_d_ctrl_l() {
        let mut editor = editor();
        type_text(&mut editor, "Eval 1.");
        let ctrl = |c| key(KeyCode::Char(c), KeyModifiers::CONTROL);
        // Ctrl+D edits a non-empty input
        assert_eq!(editor.key(ctrl('d')), Action::None);
        assert_eq!(editor.key(ctrl('l')), Action::ClearOutputs);
        assert_eq!(editor.key(ctrl('c')), Action::None);
        assert_eq!(editor.input(), "");
        assert_eq!(editor.key(ctrl('c')), Action::Quit);
        assert_eq!(editor.key(ctrl('d')), Action::Quit);
    }

    #[test]
    fn paste_keeps_the_line_breaks() {
        let mut editor = editor();
        editor.paste("Def a := 1.\r\nEval a.");
        assert_eq!(editor.input(), "Def a := 1.\nEval a.");
        assert_eq!(editor.textarea().lines().len(), 2);
    }

    #[test]
    fn same_input_twice_kept_once() {
        let mut history = History::default();
        history.push("Eval 1.".to_string());
        history.push("Eval 1.".to_string());
        assert_eq!(history.previous(""), Some("Eval 1."));
        assert_eq!(history.previous(""), None);
    }

    #[test]
    fn history_file_one_input_per_line() {
        for input in [
            "Eval 1.",
            "Def a := 1.\nEval a.",
            "Def c := '\\\\'.",
            "a\\nb",
        ] {
            assert_eq!(decode(&encode(input)), input);
            assert!(!encode(input).contains('\n'));
        }
        let dir = std::env::temp_dir().join("startlang_history_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("history.txt");
        let history = History::from(vec![
            "Eval 1.".to_string(),
            "Def a := 1.\nEval a.".to_string(),
        ]);
        history.save(&path).unwrap();
        assert_eq!(History::load(&path), history);
        // no file: an empty history
        assert_eq!(History::load(&dir.join("none.txt")), History::default());
    }
}
