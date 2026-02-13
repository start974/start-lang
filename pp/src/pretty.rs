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

impl<T, U> Pretty for (T, U)
where
    T: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.0.pretty(theme)
    }
}

// ============================================================================
// Pretty with Precedence
// ============================================================================
pub trait PrettyPrecedence {
    /// get level of type
    fn precedence(&self) -> usize;

    /// pretty with precedence
    fn pretty_precedence(&self, min_prec: usize, theme: &Theme) -> Doc<'_>;
}

impl<T> Pretty for T
where
    T: PrettyPrecedence,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.pretty_precedence(self.precedence(), theme)
    }
}

impl<T> PrettyPrecedence for Box<T>
where
    T: PrettyPrecedence,
{
    fn precedence(&self) -> usize {
        self.as_ref().precedence()
    }

    fn pretty_precedence(&self, min_prec: usize, theme: &Theme) -> Doc<'_> {
        self.as_ref().pretty_precedence(min_prec, theme)
    }
}


