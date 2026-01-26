#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// new span
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// start position
    pub fn start(&self) -> usize {
        self.start
    }

    /// end position
    pub fn end(&self) -> usize {
        self.end
    }

    /// add offset to span
    pub fn with_offset(&self, offset: usize) -> Span {
        Self::new(self.start() + offset, self.end() + offset)
    }

    /// union of spans
    pub fn union(&self, other: Span) -> Span {
        use std::cmp::{max, min};
        Span::new(
            min(self.start(), other.start()),
            max(self.end(), other.end()),
        )
    }
}

pub trait GetSpan {
    /// get span
    fn span(&self) -> Span;
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
        self.start
    }

    fn end(&self) -> Self::Offset {
        self.end
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
        let span1 = Span::new(1, 5);
        let span2 = Span::new(3, 7);
        let union = span1.union(span2);
        assert_eq!(union, Span::new(1, 7));
    }

    #[test]
    fn with_offset() {
        let span = Span::new(2, 6);
        let offset_span = span.with_offset(3);
        assert_eq!(offset_span, Span::new(5, 9));
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
