use std::collections::VecDeque;

use pretty::{Render, RenderAnnotated};

use super::{
    theme::{ColorInfo, Doc, Theme},
    writer::StringPrettyWriter,
};

// ===========================================================================
// Pretty Trait
// ===========================================================================
pub trait Pretty: Sized {
    /// pretty print
    fn pretty(&self, theme: &Theme) -> Doc<'_>;
}

// ===========================================================================
// Pretty Writer
// ===========================================================================
pub struct PrettyWriter<W, T> {
    writer: W,
    theme: T,
}

impl<W, T> std::io::Write for PrettyWriter<W, T>
where
    W: std::io::Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W, T> PrettyWriter<W, T>
where
    W: std::io::Write,
    T: AsRef<Theme>,
{
    /// print object with pretty and theme
    pub fn print(&mut self, o: &impl Pretty) -> std::io::Result<()> {
        let theme = self.theme.as_ref();
        o.pretty(theme)
            .render_raw(theme.width, &mut StreamColored::new(&mut self.writer))?;
        self.writer.flush()
    }

    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }
}

impl<W, T> PrettyWriter<W, T> {
    pub fn new(theme: T, writer: W) -> Self {
        Self { writer, theme }
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
    W: std::io::Write,
{
    type Error = std::io::Error;

    fn write_str_all(&mut self, s: &str) -> Result<(), Self::Error> {
        if let Some(color_info) = self.color_info_stack.front() {
            let s = format!("{}", color_info.colorize(s));
            self.upstream.write_all(s.as_bytes())
        } else {
            Ok(())
        }
    }

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        self.write_str_all(s).map(|_| s.len())
    }

    fn fail_doc(&self) -> Self::Error {
        Self::Error::new(std::io::ErrorKind::Other, "fail to write doc")
    }
}

impl<W> RenderAnnotated<'_, ColorInfo> for StreamColored<'_, W>
where
    W: std::io::Write,
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
