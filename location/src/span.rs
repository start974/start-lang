#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span(usize, usize);

impl Span {
    /// new span
    pub fn new(start: usize, end: usize) -> Self {
        Self(start, end)
    }

    /// start position
    pub fn start(&self) -> usize {
        self.0
    }

    /// end position
    pub fn end(&self) -> usize {
        self.1
    }

    /// add offset to span
    pub fn with_offset(&self, offset: usize) -> Span {
        Span(self.start() + offset, self.end() + offset)
    }

    /// union of spans
    pub fn union(&self, other: Span) -> Span {
        Span(
            std::cmp::min(self.start(), other.start()),
            std::cmp::max(self.end(), other.end()),
        )
    }
}

pub trait Spanned {
    /// get span
    fn span(&self) -> Span;
}

pub trait SpannedSet: Sized {
    /// set span
    fn set_span(&mut self, span: Span);

    /// with span
    fn with_span(mut self, span: Span) -> Self {
        self.set_span(span);
        self
    }

    /// with item spanned
    fn with_spanned(self, x: &impl Spanned) -> Self {
        self.with_span(x.span())
    }
}

impl<T> Spanned for (T, Span) {
    fn span(&self) -> Span {
        self.1
    }
}

// ============================================================================
// Test
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

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
}
