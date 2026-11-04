//! The state of `startlang tui` and its drawing, without terminal: the
//! inputs run so far, the input being typed, the help.
//!
//! ```text
//! ┌ input 1 ─────────────────────────┐  ← an input: its name
//! │   1 │ Def a := 3.                │  ← its code formatted, numbered
//! │       ┌────────────────────────┐ │
//! │       │a : ℕ                   │ │  ← the summary of the command (green)
//! │       └────────────────────────┘ │
//! │   2 │ Eval a.                    │
//! │       ┌────────────────────────┐ │
//! │       │3 : ℕ                   │ │
//! │       └────────────────────────┘ │
//! └──────────────────────────────────┘
//! ┌ input 2 ─────────────────────────┐
//! │   1 │ Eval   b.                  │  ← an input with errors: as typed
//! │       ┌────────────────────────┐ │
//! │       │[301] Error: …          │ │  ← its errors (red)
//! │       └────────────────────────┘ │
//! └──────────────────────────────────┘
//! ┌ input ───────────────────────────┐
//! │ 1 Def c := 'c'.                  │  ← the input area
//! └──────────────────────────────────┘
//!  Enter run · Shift+Enter / Alt+Enter new line · ↑ ↓ history · Alt+H help
//! ```

use super::editor::{Action, Editor, History, KEYS};
use super::output::{NUMBERS, Output};
use super::session::Session;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph, Widget};
use std::cell::Cell;

// ============================================================================
// App
// ============================================================================

/// lines scrolled by a notch of the mouse wheel
const WHEEL: u16 = 3;

/// the main keys, on the status line (all of them in the help)
const HINT: &str = "Enter run · Shift+Enter / Alt+Enter new line · ↑ ↓ history · \
                    PageUp / PageDown scroll · F1 / Alt+H help · Ctrl+D quit";

/// the TUI: a session, its outputs, the input area, the help
pub struct App {
    session: Session,
    editor: Editor,
    outputs: Vec<Output>,
    /// lines the outputs are scrolled up from the newest (0: at the bottom)
    scroll: u16,
    /// the height of the output area and how far it can scroll, as last
    /// drawn: the step and the limit of PageUp / PageDown
    view: Cell<(u16, u16)>,
    /// the help is shown over the rest
    help: bool,
    quit: bool,
}

impl App {
    pub fn new(history: History) -> Self {
        Self {
            session: Session::default(),
            editor: Editor::from(history),
            outputs: Vec::new(),
            scroll: 0,
            view: Cell::new((0, 0)),
            help: false,
            quit: false,
        }
    }

    /// the user asked to quit
    pub fn quit(&self) -> bool {
        self.quit
    }

    /// the inputs run, to save them
    pub fn history(&self) -> &History {
        self.editor.history()
    }

    /// handle an event of the terminal: a key, a paste
    pub fn event(&mut self, event: Event) {
        match event {
            Event::Key(key) => self.key(key),
            Event::Paste(text) if !self.help => self.editor.paste(&text),
            // the wheel scrolls the outputs, wherever the mouse is
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => self.scroll_up(WHEEL),
                MouseEventKind::ScrollDown => self.scroll_down(WHEEL),
                _ => {}
            },
            _ => {}
        }
    }

    fn key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Release {
            return;
        }
        let alt = key.modifiers.contains(KeyModifiers::ALT);
        match key.code {
            // F1 is kept by some terminals (GNOME Terminal): Alt+H too
            KeyCode::F(1) => self.help = !self.help,
            KeyCode::Char('h') if alt => self.help = !self.help,
            KeyCode::Esc if self.help => self.help = false,
            // the help is over the input: keys do not reach it
            _ if self.help => {}
            // half the output area at a time
            KeyCode::PageUp => self.scroll_up((self.view.get().0 / 2).max(1)),
            KeyCode::PageDown => self.scroll_down((self.view.get().0 / 2).max(1)),
            _ => match self.editor.key(key) {
                Action::None => {}
                Action::Run(input) => self.run(&input),
                Action::Quit => self.quit = true,
                Action::ClearOutputs => {
                    self.outputs.clear();
                    self.scroll = 0;
                }
            },
        }
    }

    /// scroll the outputs up by `lines`, up to the oldest
    fn scroll_up(&mut self, lines: u16) {
        self.scroll = (self.scroll + lines).min(self.view.get().1);
    }

    /// scroll the outputs down by `lines`, down to the newest
    fn scroll_down(&mut self, lines: u16) {
        self.scroll = self.scroll.saturating_sub(lines);
    }

    /// run `input`: its frame, with each command and its summary, or its
    /// errors
    fn run(&mut self, input: &str) {
        self.outputs.push(self.session.run(input));
        // back to the newest
        self.scroll = 0;
    }

    /// draw the whole TUI in `frame`
    pub fn draw(&self, frame: &mut Frame) {
        let input = self.editor.textarea().lines().len() as u16 + 2;
        let input = input.min(frame.area().height / 2);
        let [outputs, input, status] = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(input),
            Constraint::Length(1),
        ])
        .areas(frame.area());
        self.draw_outputs(outputs, frame.buffer_mut());
        self.draw_input(input, frame);
        frame.render_widget(Line::from(HINT).dim(), status);
        if self.help {
            self.draw_help(frame);
        }
    }

    /// the input area: the text area, its line numbers with a separator
    /// (`   1 │ `) on the left, following its scrolling
    fn draw_input(&self, area: Rect, frame: &mut Frame) {
        let block = Block::bordered().title(" input ");
        let [numbers, text] = Layout::horizontal([Constraint::Length(NUMBERS), Constraint::Min(1)])
            .areas(block.inner(area));
        frame.render_widget(block, area);
        let textarea = self.editor.textarea();
        frame.render_widget(textarea, text);
        // the first line shown, known once the text area is drawn
        let (top, _) = textarea.scroll_offset();
        let count = textarea.lines().len();
        let lines: Vec<Line> = (usize::from(top)..count)
            .take(usize::from(numbers.height))
            .map(|row| Line::from(format!("{:>4} │ ", row + 1)).dim())
            .collect();
        frame.render_widget(Paragraph::new(lines), numbers);
    }

    /// the outputs, the newest at the bottom, scrolled up by `scroll`: drawn
    /// whole in a buffer of their height, then the lines shown copied (a
    /// block cut at the top is still drawn right)
    fn draw_outputs(&self, area: Rect, buf: &mut Buffer) {
        // shortcut: no wrapping of long lines; add a wrapped height if
        // outputs get long lines
        let height: u16 = self.outputs.iter().map(Output::height).sum();
        let all = Rect::new(0, 0, area.width, height);
        let mut drawn = Buffer::empty(all);
        let mut y = 0;
        for output in &self.outputs {
            let block = Rect::new(0, y, area.width, output.height());
            output.render(block, &mut drawn);
            y += output.height();
        }
        let shown = height.min(area.height);
        let max = height - shown;
        self.view.set((area.height, max));
        let first = max - self.scroll.min(max);
        for row in 0..shown {
            for col in 0..area.width {
                let cell = drawn[(col, first + row)].clone();
                buf[(area.x + col, area.y + row)] = cell;
            }
        }
    }

    /// the keys, over the rest: a column of keys, a column of actions, each
    /// as wide as its longest entry
    fn draw_help(&self, frame: &mut Frame) {
        let width = |text: &str| Span::raw(text).width();
        let keys = KEYS.iter().map(|(key, _)| width(key)).max().unwrap_or(0);
        let actions = KEYS
            .iter()
            .map(|(_, action)| width(action))
            .max()
            .unwrap_or(0);
        let lines: Vec<Line> = KEYS
            .iter()
            .map(|(key, action)| {
                let key = format!("{key}{}", " ".repeat(keys + 2 - width(key)));
                Line::from(vec![key.bold(), (*action).into()])
            })
            .collect();
        // the two columns, two spaces apart, and the borders
        let size = |n: usize| u16::try_from(n).unwrap_or(u16::MAX);
        let area = frame.area().centered(
            Constraint::Length(size(keys + 2 + actions + 2)),
            Constraint::Length(size(KEYS.len() + 2)),
        );
        frame.render_widget(Clear, area);
        let block = Block::bordered()
            .title(" help ")
            .border_style(Style::new().fg(Color::Cyan));
        frame.render_widget(Paragraph::new(lines).block(block), area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn app() -> App {
        App::new(History::default())
    }

    fn press(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
        app.event(Event::Key(KeyEvent::new(code, modifiers)));
    }

    /// type `input` and run it
    fn run(app: &mut App, input: &str) {
        app.event(Event::Paste(input.to_string()));
        press(app, KeyCode::Enter, KeyModifiers::NONE);
    }

    /// the screen of `app`, `width` × `height`, as lines of text
    fn screen(app: &App, width: u16, height: u16) -> Vec<String> {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| app.draw(frame)).unwrap();
        let buf = terminal.backend().buffer();
        (0..height)
            .map(|y| (0..width).map(|x| buf[(x, y)].symbol()).collect::<String>())
            .collect()
    }

    /// the color of the left border of the sub-frame holding `text`
    fn border_left_of(app: &App, text: &str, width: u16, height: u16) -> Option<Color> {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| app.draw(frame)).unwrap();
        let lines = screen(app, width, height);
        let y = lines.iter().position(|line| line.contains(text))?;
        // the left border of the sub-frame: after the line numbers
        let x = 1 + NUMBERS as usize;
        assert_eq!(lines[y].chars().nth(x), Some('│'), "{lines:#?}");
        Some(terminal.backend().buffer()[(x as u16, y as u16)].fg)
    }

    #[test]
    fn an_input_in_a_frame_with_its_name_and_line_numbers() {
        let mut app = app();
        run(&mut app, "Def a := 3.\nEval a.");
        let lines = screen(&app, 40, 16);
        assert!(lines[0].starts_with("┌ input 1 "), "{lines:#?}");
        assert!(lines[1].starts_with("│   1 │ Def a := 3."), "{lines:#?}");
        // the summary of each command under it, in a green sub-frame
        assert!(lines[3].contains("│a : ℕ"), "{lines:#?}");
        assert!(lines[5].starts_with("│   2 │ Eval a."), "{lines:#?}");
        assert!(lines[7].contains("│3 : ℕ"), "{lines:#?}");
        assert_eq!(border_left_of(&app, "a : ℕ", 40, 16), Some(Color::Green));
    }

    #[test]
    fn an_input_with_errors_and_its_report_in_a_red_frame() {
        let mut app = app();
        run(&mut app, "Eval b.");
        let lines = screen(&app, 80, 30);
        assert!(lines[0].starts_with("┌ input 1 "), "{lines:#?}");
        assert!(lines[1].starts_with("│   1 │ Eval b."), "{lines:#?}");
        assert!(
            lines.iter().any(|line| line.contains("[301] Error")),
            "{lines:#?}"
        );
        assert!(
            lines.iter().any(|line| line.contains("input 1:1:6")),
            "{lines:#?}"
        );
        assert_eq!(
            border_left_of(&app, "[301] Error", 80, 30),
            Some(Color::Red)
        );
    }

    #[test]
    fn the_newest_outputs_at_the_bottom() {
        let mut app = app();
        for i in 0..10 {
            run(&mut app, &format!("Eval {i}."));
        }
        // 6 lines a frame: the last one shown, the first ones scrolled out
        let lines = screen(&app, 30, 12);
        assert!(
            lines.iter().any(|line| line.contains("input 10")),
            "{lines:#?}"
        );
        assert!(
            lines.iter().any(|line| line.contains("9 : ℕ")),
            "{lines:#?}"
        );
        assert!(
            !lines.iter().any(|line| line.contains("input 1 ")),
            "{lines:#?}"
        );
        // the input area and the status line below
        assert!(lines[8].contains("input"), "{lines:#?}");
        assert!(lines[11].starts_with("Enter run"), "{lines:#?}");
    }

    #[test]
    fn help_with_f1_or_alt_h() {
        for (code, modifiers) in [
            (KeyCode::F(1), KeyModifiers::NONE),
            (KeyCode::Char('h'), KeyModifiers::ALT),
        ] {
            let mut app = app();
            press(&mut app, code, modifiers);
            let lines = screen(&app, 80, 24);
            // the popup: its title and the keys
            assert!(
                lines.iter().any(|line| line.contains("┌ help ")),
                "{lines:#?}"
            );
            assert!(
                lines
                    .iter()
                    .any(|line| line.contains("Shift+Enter, Alt+Enter")),
                "{lines:#?}"
            );
            // the keys do not reach the input while the help is shown
            press(&mut app, KeyCode::Char('x'), KeyModifiers::NONE);
            assert_eq!(app.editor.input(), "");
            // the same key closes it
            press(&mut app, code, modifiers);
            assert!(!app.help);
        }
        // Esc closes it too
        let mut app = app();
        press(&mut app, KeyCode::F(1), KeyModifiers::NONE);
        press(&mut app, KeyCode::Esc, KeyModifiers::NONE);
        assert!(!app.help);
    }

    #[test]
    fn help_two_columns_sized_by_the_keys() {
        let mut app = app();
        press(&mut app, KeyCode::F(1), KeyModifiers::NONE);
        let lines = screen(&app, 100, 30);
        // the column (in characters) of `text` on the line of `key`
        let column = |key: &str, text: &str| {
            let line = lines
                .iter()
                .find(|line| line.contains(&format!("│{key} ")))
                .unwrap_or_else(|| panic!("{key}: {lines:#?}"));
            let byte = line.find(text).unwrap();
            line[..byte].chars().count()
        };
        // every action starts in the same column
        let actions: Vec<_> = KEYS
            .iter()
            .map(|(key, action)| column(key, action))
            .collect();
        assert!(actions.iter().all(|c| *c == actions[0]), "{lines:#?}");
        // two spaces after the longest key
        let keys = KEYS
            .iter()
            .map(|(key, _)| key.chars().count())
            .max()
            .unwrap();
        assert_eq!(actions[0] - column(KEYS[0].0, KEYS[0].0), keys + 2);
        // the frame as wide as the two columns and its borders
        let widest = KEYS
            .iter()
            .map(|(_, action)| action.chars().count())
            .max()
            .unwrap();
        let top = lines.iter().find(|line| line.contains("┌ help ")).unwrap();
        let frame = top.trim().chars().count();
        assert_eq!(frame, keys + 2 + widest + 2, "{lines:#?}");
    }

    #[test]
    fn input_line_numbers_with_a_separator() {
        let mut app = app();
        app.event(Event::Paste("Def a := 1.\nEval a.".to_string()));
        let lines = screen(&app, 40, 12);
        assert!(lines[8].starts_with("│   1 │ Def a := 1."), "{lines:#?}");
        assert!(lines[9].starts_with("│   2 │ Eval a."), "{lines:#?}");
    }

    #[test]
    fn input_line_numbers_follow_its_scrolling() {
        let mut app = app();
        // 10 lines in an input area of 4 (half the screen, less its borders)
        let input: Vec<_> = (1..=10).map(|i| format!("Eval {i}.")).collect();
        app.event(Event::Paste(input.join("\n")));
        let lines = screen(&app, 40, 12);
        // the cursor on the last line: the last lines shown, numbered
        assert!(
            lines
                .iter()
                .any(|line| line.starts_with("│  10 │ Eval 10.")),
            "{lines:#?}"
        );
        assert!(
            !lines.iter().any(|line| line.contains("│   1 │")),
            "{lines:#?}"
        );
    }

    #[test]
    fn page_up_and_down_scroll_the_outputs() {
        let mut app = app();
        for i in 0..10 {
            run(&mut app, &format!("Eval {i}."));
        }
        let shows =
            |app: &App, text: &str| screen(app, 30, 12).iter().any(|line| line.contains(text));
        assert!(shows(&app, "input 10") && !shows(&app, "input 1 "));
        // up to the oldest, and no further
        for _ in 0..20 {
            press(&mut app, KeyCode::PageUp, KeyModifiers::NONE);
            screen(&app, 30, 12);
        }
        assert!(shows(&app, "input 1 ") && !shows(&app, "input 10"));
        // down to the newest, and no further
        for _ in 0..20 {
            press(&mut app, KeyCode::PageDown, KeyModifiers::NONE);
        }
        assert_eq!(app.scroll, 0);
        assert!(shows(&app, "input 10"));
        // a new input brings the view back to the newest
        press(&mut app, KeyCode::PageUp, KeyModifiers::NONE);
        assert!(app.scroll > 0);
        run(&mut app, "Eval 11.");
        assert_eq!(app.scroll, 0);
    }

    #[test]
    fn the_wheel_scrolls_the_outputs_not_the_history() {
        use crossterm::event::MouseEvent;
        let mut app = app();
        for i in 0..10 {
            run(&mut app, &format!("Eval {i}."));
        }
        screen(&app, 30, 12);
        let wheel = |kind| {
            Event::Mouse(MouseEvent {
                kind,
                column: 0,
                row: 0,
                modifiers: KeyModifiers::NONE,
            })
        };
        app.event(wheel(MouseEventKind::ScrollUp));
        assert_eq!(app.scroll, WHEEL);
        // the input is left as it is
        assert_eq!(app.editor.input(), "");
        app.event(wheel(MouseEventKind::ScrollDown));
        assert_eq!(app.scroll, 0);
    }

    #[test]
    fn ctrl_l_clears_ctrl_d_quits() {
        let mut app = app();
        run(&mut app, "Eval 1.");
        press(&mut app, KeyCode::Char('l'), KeyModifiers::CONTROL);
        assert!(app.outputs.is_empty());
        press(&mut app, KeyCode::Char('d'), KeyModifiers::CONTROL);
        assert!(app.quit());
        assert_eq!(app.history(), &History::from(vec!["Eval 1.".to_string()]));
    }
}
