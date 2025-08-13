use super::theme::{ColorInfo, Doc, Theme};
use pretty::{Render, RenderAnnotated};
use std::collections::VecDeque;

// ===========================================================================
// Pretty Trait
// ===========================================================================
pub trait Pretty: Sized {
    /// pretty print
    fn pretty(&self, theme: &Theme) -> Doc<'_>;

    /// write with fmt
    fn fmt(&self, theme: &Theme, fmt: &mut impl std::fmt::Write) -> std::fmt::Result {
        let mut stream = StreamColored::new(fmt);
        self.pretty(theme).render_raw(theme.width, &mut stream)
    }

    /// get colored string
    fn make_string(&self, theme: &Theme) -> String {
        let mut buffer = String::new();
        self.fmt(theme, &mut buffer).unwrap();
        buffer
    }
}

// ===========================================================================
// Stream Colored
// ===========================================================================
pub struct StreamColored<'w, W> {
    upstream: &'w mut W,
    color_info_stack: VecDeque<ColorInfo>,
}

impl<'w, W> StreamColored<'w, W> {
    pub fn new(upstream: &'w mut W) -> Self {
        Self {
            color_info_stack: VecDeque::new(),
            upstream,
        }
    }
}

impl<W> Render for StreamColored<'_, W>
where
    W: std::fmt::Write,
{
    type Error = std::fmt::Error;

    fn write_str_all(&mut self, s: &str) -> Result<(), Self::Error> {
        if let Some(color_info) = self.color_info_stack.front() {
            write!(self.upstream, "{}", color_info.colorize(s))
        } else {
            self.upstream.write_str(s)
        }
    }

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        self.write_str_all(s).map(|()| s.len())
    }

    fn fail_doc(&self) -> Self::Error {
        std::fmt::Error
    }
}

impl<W> RenderAnnotated<'_, ColorInfo> for StreamColored<'_, W>
where
    W: std::fmt::Write,
{
    fn push_annotation(&mut self, annot: &ColorInfo) -> Result<(), Self::Error> {
        self.color_info_stack.push_front(annot.clone());
        Ok(())
    }

    fn pop_annotation(&mut self) -> Result<(), Self::Error> {
        self.color_info_stack
            .pop_front()
            .map(|_| ())
            .ok_or_else(|| self.fail_doc())
    }
}
