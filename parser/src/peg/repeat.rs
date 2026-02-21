use location::{GetSpan, SetSpan, Span};
use pp::pretty::*;

#[derive(Debug, Clone)]
pub struct Repeat {
    /// The minimum number of repetitions (0 for `*`, 1 for `+`).
    min: usize,

    /// The maximum number of repetitions (None for unlimited).
    max: Option<usize>,

    /// span of operator `?`, `*`, `+`or {n, m}
    span: Span,
}

impl Repeat {
    /// optional repetition, e.g. `a?`
    pub fn optional() -> Self {
        Repeat {
            min: 0,
            max: Some(1),
            span: Span::default(),
        }
    }

    /// zero or more repetition, e.g. `a*`
    pub fn zero_or_more() -> Self {
        Repeat {
            min: 0,
            max: None,
            span: Span::default(),
        }
    }

    /// one or more repetition, e.g. `a+`
    pub fn one_or_more() -> Self {
        Repeat {
            min: 1,
            max: None,
            span: Span::default(),
        }
    }

    /// exactly n repetitions, e.g. `a{3}`
    pub fn exactly(n: usize) -> Self {
        Repeat {
            min: n,
            max: Some(n),
            span: Span::default(),
        }
    }

    /// between min and max repetitions, e.g. `a{2,5}`
    /// neded to check that min <= max
    pub fn between(min: usize, max: usize) -> Self {
        assert!(
            min <= max,
            "Minimum repetitions cannot be greater than maximum"
        );
        Repeat {
            min,
            max: Some(max),
            span: Span::default(),
        }
    }

    /// at least min repetitions, e.g. `a{2,}`
    pub fn at_least(min: usize) -> Self {
        Repeat {
            min,
            max: None,
            span: Span::default(),
        }
    }

    /// at most max repetitions, e.g. `a{,5}`
    pub fn at_most(max: usize) -> Self {
        Repeat {
            min: 0,
            max: Some(max),
            span: Span::default(),
        }
    }

    /// get minimum repetitions
    pub fn min(&self) -> usize {
        self.min
    }

    /// get maximum repetitions
    pub fn max(&self) -> Option<usize> {
        self.max
    }

    pub fn with_opt_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}
impl PartialEq for Repeat {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}
impl Eq for Repeat {}

impl std::hash::Hash for Repeat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.min.hash(state);
        self.max.hash(state);
    }
}

impl GetSpan for Repeat {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Repeat {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
impl Pretty for Repeat {
    fn pretty(&self, _theme: &Theme) -> Doc<'_> {
        match (self.min, self.max) {
            (0, Some(1)) => Doc::text("?"),
            (0, None) => Doc::text("*"),
            (1, None) => Doc::text("+"),
            (n, Some(m)) if n == m => Doc::text("{")
                .append(Doc::as_string(n))
                .append(Doc::text("}")),
            (n, Some(m)) => Doc::text("{")
                .append(Doc::as_string(n))
                .append(Doc::text(","))
                .append(Doc::space())
                .append(Doc::as_string(m))
                .append(Doc::text("}")),
            (n, None) => Doc::text("{")
                .append(Doc::as_string(n))
                .append(Doc::text(",}")),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn pretty() {
        let theme = Theme::default();
        {
            let optional = Repeat::optional();
            assert_eq!(optional.make_string(&theme), "?");
        }
        {
            let zero_or_more = Repeat::zero_or_more();
            assert_eq!(zero_or_more.make_string(&theme), "*");
        }
        {
            let one_or_more = Repeat::one_or_more();
            assert_eq!(one_or_more.make_string(&theme), "+");
        }
        {
            let exactly = Repeat::exactly(3);
            assert_eq!(exactly.make_string(&theme), "{3}");
        }
        {
            let range = Repeat::between(2, 5);
            assert_eq!(range.make_string(&theme), "{2, 5}");
        }
        {
            let zero_or_more = Repeat::zero_or_more();
            assert_eq!(zero_or_more.make_string(&theme), "*");
        }
    }
}
