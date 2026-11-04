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
//!  Enter run · Ctrl+Enter / Ctrl+U new line · ↑ ↓ history · F1 help
//! ```

use super::editor::{Action, Editor, History, KEYS};
use super::output::Output;
use super::session::Session;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear, Paragraph, Widget};

// ============================================================================
// App
// ============================================================================

/// the TUI: a session, its outputs, the input area, the help
pub struct App {
    session: Session,
    editor: Editor,
    outputs: Vec<Output>,
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
            _ => {}
        }
    }

    fn key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Release {
            return;
        }
        match key.code {
            KeyCode::F(1) => self.help = !self.help,
            KeyCode::Esc if self.help => self.help = false,
            // the help is over the input: keys do not reach it
            _ if self.help => {}
            _ => match self.editor.key(key) {
                Action::None => {}
                Action::Run(input) => self.run(&input),
                Action::Quit => self.quit = true,
                Action::ClearOutputs => self.outputs.clear(),
            },
        }
    }

    /// run `input`: its frame, with each command and its summary, or its
    /// errors
    fn run(&mut self, input: &str) {
        self.outputs.push(self.session.run(input));
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
        let block = Block::bordered().title(" input ");
        frame.render_widget(self.editor.textarea(), block.inner(input));
        frame.render_widget(block, input);
        let hint = "Enter run · Ctrl+Enter / Ctrl+U new line · ↑ ↓ history · F1 help · Ctrl+D quit";
        frame.render_widget(Line::from(hint).dim(), status);
        if self.help {
            self.draw_help(frame);
        }
    }

    /// the outputs, the newest at the bottom: drawn whole in a buffer of
    /// their height, then its bottom copied (a block cut at the top is
    /// still drawn right)
    fn draw_outputs(&self, area: Rect, buf: &mut Buffer) {
        // shortcut: no scrolling back and no wrapping of long lines; add
        // PageUp / PageDown and a wrapped height if outputs get long
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
        for row in 0..shown {
            for col in 0..area.width {
                let cell = drawn[(col, height - shown + row)].clone();
                buf[(area.x + col, area.y + row)] = cell;
            }
        }
    }

    /// the keys, over the rest
    fn draw_help(&self, frame: &mut Frame) {
        let lines: Vec<Line> = KEYS
            .iter()
            .map(|(key, action)| Line::from(vec![format!("{key:<20}").bold(), (*action).into()]))
            .collect();
        let height = lines.len() as u16 + 2;
        let area = frame
            .area()
            .centered(Constraint::Length(72), Constraint::Length(height));
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
    use crate::tui::output::NUMBERS;
    use crossterm::event::KeyModifiers;
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
    fn help_with_f1() {
        let mut app = app();
        press(&mut app, KeyCode::F(1), KeyModifiers::NONE);
        let lines = screen(&app, 80, 24);
        assert!(lines.iter().any(|line| line.contains("help")), "{lines:#?}");
        assert!(
            lines.iter().any(|line| line.contains("Ctrl+Enter, Ctrl+U")),
            "{lines:#?}"
        );
        // the keys do not reach the input while the help is shown
        press(&mut app, KeyCode::Char('x'), KeyModifiers::NONE);
        assert_eq!(app.editor.input(), "");
        press(&mut app, KeyCode::Esc, KeyModifiers::NONE);
        assert!(!app.help);
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
