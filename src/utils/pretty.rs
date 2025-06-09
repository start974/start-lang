use std::collections::VecDeque;

use pretty::{Render, RenderAnnotated};

use super::theme::{ColorInfo, Doc, Theme, ThemeGet};
use super::writer::WriterTrait;

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

impl<W, T> std::fmt::Write for PrettyWriter<W, T>
where
    W: std::fmt::Write,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.writer.write_str(s)
    }
}

impl<W, T> WriterTrait for PrettyWriter<W, T> where W: WriterTrait {}

impl<W, T> ThemeGet for PrettyWriter<W, T>
where
    T: AsRef<Theme>,
{
    fn theme(&self) -> &Theme {
        self.theme.as_ref()
    }
}

impl<W, T> PrettyWriter<W, T>
where
    W: std::fmt::Write,
    T: AsRef<Theme>,
{
    /// print object with pretty and theme
    pub fn print(&mut self, o: &impl Pretty) {
        let doc = o.pretty(self.theme());
        let width = self.theme().width;
        let mut stream = StreamColored::new(&mut self.writer);
        doc.render_raw(width, &mut stream).unwrap();
    }
}

impl<W, T> PrettyWriter<W, T>
where
    W: Default,
    T: AsRef<Theme>,
{
    pub fn new(theme: T) -> Self {
        Self {
            writer: W::default(),
            theme,
        }
    }
}

impl<T> PrettyWriter<String, T> {
    /// get string from writer
    pub fn get_string(self) -> String {
        self.writer
    }

    /// clear the writer
    pub fn clear(&mut self) {
        self.writer.clear();
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
            write!(&mut self.upstream, "{}", color_info.colorize(s))
        } else {
            write!(&mut self.upstream, "{}", s)
        }
    }

    fn write_str(&mut self, s: &str) -> Result<usize, Self::Error> {
        self.write_str_all(s).map(|_| s.len())
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
        self.color_info_stack.pop_front();
        Ok(())
    }
}
