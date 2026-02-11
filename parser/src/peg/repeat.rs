use location::{GetSpan, Span};

use super::Peg;

#[derive(Debug)]
pub struct Repeat {
    /// The PEG rule that is being repeated.
    peg: Box<Peg>,

    /// The minimum number of repetitions (0 for `*`, 1 for `+`).
    min: usize,

    /// The maximum number of repetitions (None for unlimited).
    max: Option<usize>,

    /// span of operator `?`, `*`, `+`or {n, m}
    span_op: Span,
}

impl Repeat {
    /// optional repetition, e.g. `a?`
    pub fn optional(peg: Peg) -> Self {
        Repeat {
            peg: Box::new(peg),
            min: 0,
            max: Some(1),
            span_op: Span::default(),
        }
    }

    /// zero or more repetition, e.g. `a*`
    pub fn zero_or_more(peg: Peg) -> Self {
        Repeat {
            peg: Box::new(peg),
            min: 0,
            max: None,
            span_op: Span::default(),
        }
    }

    /// one or more repetition, e.g. `a+`
    pub fn one_or_more(peg: Peg) -> Self {
        Repeat {
            peg: Box::new(peg),
            min: 1,
            max: None,
            span_op: Span::default(),
        }
    }

    /// exactly n repetitions, e.g. `a{3}`
    pub fn exactly(peg: Peg, n: usize) -> Self {
        Repeat {
            peg: Box::new(peg),
            min: n,
            max: Some(n),
            span_op: Span::default(),
        }
    }

    /// between min and max repetitions, e.g. `a{2,5}`
    /// neded to check that min <= max
    pub fn between(peg: Peg, min: usize, max: usize) -> Self {
        assert!(
            min <= max,
            "Minimum repetitions cannot be greater than maximum"
        );
        Repeat {
            peg: Box::new(peg),
            min,
            max: Some(max),
            span_op: Span::default(),
        }
    }

    /// at least min repetitions, e.g. `a{2,}`
    pub fn at_least(peg: Peg, min: usize) -> Self {
        Repeat {
            peg: Box::new(peg),
            min,
            max: None,
            span_op: Span::default(),
        }
    }

    /// at most max repetitions, e.g. `a{,5}`
    pub fn at_most(peg: Peg, max: usize) -> Self {
        Repeat {
            peg: Box::new(peg),
            min: 0,
            max: Some(max),
            span_op: Span::default(),
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

    /// get peg rule
    pub fn rule(&self) -> &Peg {
        &self.peg
    }

    pub fn with_opt_span(mut self, span: Span) -> Self {
        self.span_op = span;
        self
    }
}

impl GetSpan for Repeat {
    fn span(&self) -> Span {
        self.peg.span().union(self.span_op)
    }
}
