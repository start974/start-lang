use crate::StreamColored;

pub use crate::{Doc, Theme};

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

impl<T> Pretty for Box<T>
where
    T: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.as_ref().pretty(theme)
    }
}
