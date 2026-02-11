mod class;
mod literal;
mod ref_rule;
mod repeat;

pub use class::*;
pub use literal::*;
use location::{GetSpan, Span};
pub use ref_rule::*;
pub use repeat::*;

/// Represents a grammar syntax node for PEG parsing and Pratt operators.
#[derive(Debug)]
pub enum Peg {
    /// A literal string, e.g. 'a', '1', or "abc".
    Literal(Literal),

    /// A character class, e.g. [a-z], [^0-9].
    Class(Class),

    /// Reference to another grammar rule by name.
    RuleRef(RefRule),

    /// Sequence of syntaxes, e.g. `a b c`.
    Seq(Vec<Peg>),

    /// Choice between multiple syntaxes, e.g. `a / b / c`.
    Choice(Vec<Peg>),

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

impl GetSpan for Peg {
    fn span(&self) -> Span {
        match self {
            Peg::Literal(literal) => literal.span(),
            Peg::Class(class) => class.span(),
            Peg::RuleRef(ref_rule) => ref_rule.span(),
            Peg::Seq(pegs) | Peg::Choice(pegs) => Span::from_iter(pegs),
            Peg::Repeat(repeat) => repeat.span(),
            Peg::NegativeLookahead(peg) | Peg::PositiveLookahead(peg) => peg.span(),
        }
    }
}
