mod class;
mod repeat;

pub use class::*;
pub use repeat::*;
use location::{GetSpan, SetSpan, Span};

/// Represents a grammar syntax node for PEG parsing and Pratt operators.
#[derive(Debug)]
pub enum Kind {
    /// A literal string, e.g. 'a', '1', or "abc".
    Literal(String),

    /// A character class, e.g. [a-z], [^0-9].
    Class(Class),

    /// Reference to another grammar rule by name.
    RuleRef(String),

    /// Sequence of syntaxes, e.g. `a b c`.
    Seq(Vec<Peg>),

    /// Choice between multiple syntaxes, e.g. `a / b / c`.
    Choice(Vec<Peg>),

    /// Grouped syntax, e.g. `(a b c)`.
    Group(Box<Peg>),

    /// Repetition with a specific range.
    /// Examples:
    /// - a? optional
    /// - a* zero or more times
    /// - a+ one or more times
    /// - `a{3}`: exactly 3 times
    /// - `a{2,5}`: between 2 and 5 times
    /// - `a{2,}`: 2 or more times
    /// - `a{,5}`: up to 5 timesa
    Repeat(Repeat),

    /// Negative lookahead: matches if the inner syntax does NOT match,
    /// without consuming input, e.g. `!a`.
    NegativeLookahead(Box<Peg>),

    /// Positive lookahead: matches if the inner syntax matches,
    /// without consuming input, e.g. `&a`.
    PositiveLookahead(Box<Peg>),
}

/// represents a PEG syntax node with its kind and span for error reporting.
#[derive(Debug)]
pub struct Peg {
    /// kind of the syntax node, e.g. literal, class, rule reference, sequence, choice, etc.
    kind: Kind,
    /// span of the syntax in the input, used for error reporting
    span: Span,
}

impl Peg {
    /// get the kind of the syntax node
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

impl From<Kind> for Peg {
    fn from(kind: Kind) -> Self {
        Peg {
            kind,
            span: Span::default(),
        }
    }
}

impl GetSpan for Peg {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Peg {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
