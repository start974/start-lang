mod class;
mod pratt;

pub use class::*;
pub use pratt::*;

/// Represents a grammar syntax node for PEG parsing and Pratt operators.
#[derive(Debug, Clone)]
pub enum Syntax {
    /// A literal string, e.g. 'a', '1', or "abc".
    Literal(String),

    /// A character class, e.g. [a-z], [^0-9].
    Class(Class),

    /// Reference to another grammar rule by name.
    RuleRef(String),

    /// Sequence of syntaxes, e.g. `a b c`.
    Seq(Vec<Syntax>),

    /// Choice between multiple syntaxes, e.g. `a / b / c`.
    Choice(Vec<Syntax>),

    /// Grouped syntax, e.g. `(a b c)`.
    Group(Box<Syntax>),

    /// Optional syntax, e.g. `a?`.
    Optional(Box<Syntax>),

    /// Zero-or-more repetition, e.g. `a*`.
    Repeat0(Box<Syntax>),

    /// One-or-more repetition, e.g. `a+`.
    Repeat1(Box<Syntax>),

    /// Repetition with a specific range.
    /// Examples:
    /// - `a{2,5}`: between 2 and 5 times
    /// - `a{3}`: exactly 3 times
    /// - `a{2,}`: 2 or more times
    /// - `a{,5}`: up to 5 times
    RepeatRange(Box<Syntax>, Option<usize>, Option<usize>),

    /// Negative lookahead: matches if the inner syntax does NOT match,
    /// without consuming input, e.g. `!a`.
    NegativeLookahead(Box<Syntax>),

    /// Positive lookahead: matches if the inner syntax matches,
    /// without consuming input, e.g. `&a`.
    PositiveLookahead(Box<Syntax>),

    /// A Pratt operator node, e.g. `a + b` with precedence and associativity.
    Pratt(Pratt),

    /// Template variable for parameterized rules, e.g. `<X : a>`.
    TemplateVar {
        /// Name of the template variable
        name: String,
        /// The syntax bound to this variable
        syntax: Box<Syntax>,
    },
}
