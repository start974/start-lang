pub use crate::utils::theme::*;
use pretty::{Render, RenderAnnotated};
use std::collections::VecDeque;

struct StreamColored<W> {
    upstream: W,
    color_info_stack: VecDeque<ColorInfo>,
}

impl<W> StreamColored<W> {
    fn new(upstream: W) -> Self {
        Self {
            color_info_stack: VecDeque::new(),
            upstream,
        }
    }
}

impl<W> Render for StreamColored<W>
where
    W: std::fmt::Write,
{
    type Error = std::fmt::Error;

    fn write_str_all(&mut self, s: &str) -> Result<(), Self::Error> {
        if let Some(color_info) = self.color_info_stack.front() {
            write!(self.upstream, "{}", color_info.colorize(s))
        } else {
            write!(self.upstream, "{}", s)
        }
    }

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        self.write_str_all(s).map(|_| s.len())
    }

    fn fail_doc(&self) -> Self::Error {
        std::fmt::Error
    }
}

impl<W> RenderAnnotated<'_, ColorInfo> for StreamColored<W>
where
    W: std::fmt::Write,
{
    fn push_annotation(&mut self, annot: &ColorInfo) -> Result<(), Self::Error> {
        self.color_info_stack.push_front(annot.clone());
        Ok(())
    }

    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        self.color_info_stack.pop_front();
        Ok(())
    }
}

pub trait Pretty {
    /// pretty print
    fn pretty(&self, theme: &Theme) -> Doc<'_>;

    /// render string with theme
    fn render(&self, theme: &Theme) -> String {
        let mut stream = StreamColored::new(String::new());
        self.pretty(theme)
            .render_raw(theme.width, &mut stream)
            .unwrap();
        stream.upstream
    }

    /// render to string
    fn to_string(&self) -> String {
        self.render(get_theme_no_color())
    }

    /// render to colored string
    fn to_string_colored(&self) -> String {
        self.render(&get_theme().lock().unwrap())
    }
}
