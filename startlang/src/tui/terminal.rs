//! The terminal of `startlang tui`: alternate screen, raw mode, bracketed
//! paste and keyboard enhancement while the TUI runs; the terminal is
//! restored when it stops, and on panic.

use crossterm::event::{
    DisableBracketedPaste, EnableBracketedPaste, KeyboardEnhancementFlags,
    PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use crossterm::terminal::supports_keyboard_enhancement;
use ratatui::DefaultTerminal;
use std::io::{self, stdout};

/// the terminal in the mode of the TUI; restored when dropped
pub struct Terminal {
    pub inner: DefaultTerminal,
}

impl Terminal {
    /// put the terminal in the mode of the TUI
    pub fn init() -> io::Result<Self> {
        // raw mode, alternate screen, and a panic hook that leaves them
        let inner = ratatui::try_init()?;
        set_panic_hook();
        execute!(stdout(), EnableBracketedPaste)?;
        // Ctrl+Enter is only reported with the keyboard enhancement (kitty
        // protocol); without it, Ctrl+U inserts a new line
        if supports_keyboard_enhancement().unwrap_or(false) {
            execute!(
                stdout(),
                PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
            )?;
        }
        Ok(Self { inner })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        leave_modes();
        ratatui::restore();
    }
}

/// leave what `ratatui::restore` does not: bracketed paste and keyboard
/// enhancement (popping flags never pushed is ignored by the terminal)
fn leave_modes() {
    // nothing more to do if the terminal does not answer
    let _ = execute!(stdout(), PopKeyboardEnhancementFlags, DisableBracketedPaste);
}

/// on panic, leave the modes, then the hook of `ratatui` restores the rest
fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        leave_modes();
        hook(info);
    }));
}
