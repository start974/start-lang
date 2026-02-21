#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Span {
    Range {
        start: usize,
        end: usize,
    },
    #[default]
    Unknown,
}

impl Span {
    /// new span
    pub fn new(start: usize, end: usize) -> Self {
        Self::Range { start, end }
    }

    /// start position
    pub fn start(&self) -> usize {
        match self {
            Self::Range { start, .. } => *start,
            Self::Unknown => 0,
        }
    }

    pub fn unknown() -> Self {
        Self::Unknown
    }

    /// end position
    pub fn end(&self) -> usize {
        match self {
            Self::Range { end, .. } => *end,
            Self::Unknown => 0,
        }
    }

    /// add offset to span
    pub fn with_offset(&self, offset: usize) -> Span {
        match self {
            Self::Range { start, end } => Self::new(*start + offset, *end + offset),
            Self::Unknown => Self::Unknown,
        }
    }

    /// union of spans
    pub fn union(self, other: Span) -> Span {
        match (self, other) {
            (Span::Unknown, x) | (x, Span::Unknown) => x,
            (Span::Range { start: s1, end: e1 }, Span::Range { start: s2, end: e2 }) => {
                use std::cmp::{max, min};
                Span::new(min(s1, s2), max(e1, e2))
            }
        }
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Self {
        Span::new(range.start, range.end)
    }
}

impl From<chumsky::span::SimpleSpan> for Span {
    fn from(value: chumsky::span::SimpleSpan) -> Self {
        Self::new(value.start, value.end)
    }
}

impl From<usize> for Span {
    fn from(value: usize) -> Self {
        Self::new(value, value)
    }
}

pub trait GetSpan {
    /// get span
    fn span(&self) -> Span;
}

impl<T> GetSpan for &T
where
    T: GetSpan,
{
    fn span(&self) -> Span {
        (*self).span()
    }
}

pub trait SetSpan: Sized {
    /// set span
    fn set_span(&mut self, span: Span);

    /// with span
    fn with_span(mut self, span: Span) -> Self {
        self.set_span(span);
        self
    }

    /// with item spanned
    fn with_spanned(self, x: &impl GetSpan) -> Self {
        self.with_span(x.span())
    }
}

impl<T> FromIterator<T> for Span
where
    T: GetSpan,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .map(|i| i.span())
            .reduce(Span::union)
            .unwrap_or_default()
    }
}

impl<T> GetSpan for (T, Span) {
    fn span(&self) -> Span {
        self.1
    }
}

impl chumsky::span::Span for Span {
    type Context = ();

    type Offset = usize;

    fn new(_: Self::Context, range: std::ops::Range<Self::Offset>) -> Self {
        Self::from(range)
    }

    fn context(&self) -> Self::Context {}

    fn start(&self) -> Self::Offset {
        Span::start(self)
    }

    fn end(&self) -> Self::Offset {
        Span::end(self)
    }
}

// ============================================================================
// Test
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestSpanned(Span);

    impl GetSpan for TestSpanned {
        fn span(&self) -> Span {
            self.0
        }
    }

    impl SetSpan for TestSpanned {
        fn set_span(&mut self, span: Span) {
            self.0 = span
        }
    }

    #[test]
    fn union() {
        {
            let span1 = Span::new(1, 5);
            let span2 = Span::new(3, 7);
            let union = span1.union(span2);
            assert_eq!(union, Span::new(1, 7));
        }
        {
            let span1 = Span::new(1, 5);
            let span2 = Span::unknown();
            assert_eq!(span1.union(span2), span1);
        }
        {
            let span1 = Span::unknown();
            let span2 = Span::unknown();
            assert_eq!(span1.union(span2), Span::unknown());
        }
    }

    #[test]
    fn with_offset() {
        {
            let span = Span::new(2, 6);
            let offset_span = span.with_offset(3);
            assert_eq!(offset_span, Span::new(5, 9));
        }
        {
            let span = Span::unknown();
            let offset_span = span.with_offset(3);
            assert_eq!(offset_span, Span::unknown());
        }
    }

    #[test]
    fn spanned_trait() {
        let span0 = Span::new(4, 8);
        let mut spanned = TestSpanned(span0);
        assert_eq!(spanned.span(), span0);

        let span1 = Span::new(0, 2);
        spanned.set_span(span1);
        assert_eq!(spanned.span(), span1);

        let span2 = Span::new(5, 10);
        assert_eq!(spanned.with_span(span2).span(), span2);
    }
}
