//! The frame of an input of `startlang tui` (`Output`) and its parts: code,
//! summaries in green sub-frames, errors in a red one.

use ansi_to_tui::IntoText;
use ir::location::SourceId;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph, Widget};

/// `lines` as a height
fn height(lines: usize) -> u16 {
    u16::try_from(lines).unwrap_or(u16::MAX)
}

// ============================================================================
// Widgets
// ============================================================================

/// width of the line numbers and their separator (`   1 │ `), and
/// indentation of the sub-frames
pub(super) const NUMBERS: u16 = 7;

/// styled text, owned: the text of a part
pub(super) type Styled = Text<'static>;

/// lines of code, numbered from the line `first` (0-based): `   1 │ code`
pub(super) struct Numbered<'a> {
    pub(super) first: usize,
    pub(super) text: &'a Styled,
}

impl Widget for Numbered<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines: Vec<Line> = self
            .text
            .lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let number = Span::from(format!("{:>4} │ ", self.first + i + 1)).dim();
                Line::from_iter(std::iter::once(number).chain(line.spans.iter().cloned()))
            })
            .collect();
        Paragraph::new(lines).render(area, buf);
    }
}

/// `text` in a sub-frame of `color`, under the code: after the line numbers,
/// a column from the right border
pub(super) struct SubFrame<'a> {
    pub(super) text: &'a Styled,
    pub(super) color: Color,
}

impl Widget for SubFrame<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect {
            x: area.x + NUMBERS,
            width: area.width.saturating_sub(NUMBERS + 1),
            ..area
        };
        let block = Block::bordered().border_style(Style::new().fg(self.color));
        Paragraph::new(self.text.clone())
            .block(block)
            .render(area, buf);
    }
}

// ============================================================================
// Parts
// ============================================================================

/// a part of the frame of an input
#[derive(Debug, PartialEq)]
pub(super) enum Part {
    /// lines of code, the first one being the line `first` (0-based)
    Code { first: usize, text: Styled },
    /// the summary of a command: in a green sub-frame
    Summary(Styled),
    /// the errors of the input: in a red sub-frame
    Error(Styled),
}

impl Part {
    /// lines it takes
    pub(super) fn height(&self) -> u16 {
        match self {
            Part::Code { text, .. } => height(text.height()),
            // the border of the sub-frame
            Part::Summary(text) | Part::Error(text) => height(text.height() + 2),
        }
    }
}

impl Widget for &Part {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Part::Code { first, text } => Numbered {
                first: *first,
                text,
            }
            .render(area, buf),
            Part::Summary(text) => SubFrame {
                text,
                color: Color::Green,
            }
            .render(area, buf),
            Part::Error(text) => SubFrame {
                text,
                color: Color::Red,
            }
            .render(area, buf),
        }
    }
}

// ============================================================================
// Output
// ============================================================================

/// an input run: a frame with its source as name, its code and what it
/// gave
#[derive(Debug, PartialEq)]
pub(super) struct Output {
    /// the id of the input, a source of its own (`input 3`): its errors
    /// point inside it
    pub(super) source: SourceId,
    pub(super) parts: Vec<Part>,
}

impl Output {
    /// lines it takes: its parts and its frame
    pub(super) fn height(&self) -> u16 {
        self.parts.iter().map(Part::height).sum::<u16>() + 2
    }
}

impl Widget for &Output {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title(format!(" {} ", self.source));
        let mut inner = block.inner(area);
        block.render(area, buf);
        for part in &self.parts {
            let height = part.height();
            part.render(Rect { height, ..inner }, buf);
            inner.y += height;
        }
    }
}

/// text with ANSI colors (theme, `ariadne`) as styled text
pub(super) fn styled(ansi: &str) -> Styled {
    ansi.into_text()
        .unwrap_or_else(|_| Text::raw(ansi.to_string()))
}
