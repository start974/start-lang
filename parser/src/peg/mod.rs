mod any;
mod class;
mod eoi;
mod literal;
mod ref_rule;
mod repeat;

pub use any::*;
pub use class::*;
pub use eoi::*;
pub use literal::*;
pub use ref_rule::*;
pub use repeat::*;

use location::{GetSpan, Span};
use pp::pretty::*;

/// Represents a grammar syntax node for PEG parsing and Pratt operators.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Peg {
    /// End of input, e.g. `$`.
    Eoi(Eoi),

    /// Any character, e.g. `.`.
    Any(Any),

    /// A literal string, e.g. 'a', '1', or "abc".
    Literal(Literal),

    /// A character class, e.g. [a-z], [^0-9].
    Class(Class),

    /// Reference to another grammar rule by name.
    RefRule(RefRule),

    /// Sequence of syntaxes, e.g. `a b c`.
    Sequence(Vec<Peg>),

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
    Repetition(Box<Peg>, Repeat),

    /// lookahead: matches if the inner syntax does match or not,
    /// without consuming input, e.g. `!a` `&a`.
    Lookahead(Box<Peg>, bool),
}

impl From<Eoi> for Peg {
    fn from(eoi: Eoi) -> Self {
        Peg::Eoi(eoi)
    }
}

impl From<Any> for Peg {
    fn from(any: Any) -> Self {
        Peg::Any(any)
    }
}

impl From<Literal> for Peg {
    fn from(literal: Literal) -> Self {
        Peg::Literal(literal)
    }
}

impl From<Class> for Peg {
    fn from(class: Class) -> Self {
        Peg::Class(class)
    }
}

impl From<RefRule> for Peg {
    fn from(ref_rule: RefRule) -> Self {
        Peg::RefRule(ref_rule)
    }
}

impl Peg {
    /// make any
    pub fn any() -> Self {
        Peg::Any(Any::default())
    }

    /// make end of input
    pub fn eoi() -> Self {
        Peg::Eoi(Eoi::default())
    }

    /// make repetition
    pub fn repetition(self, rep: Repeat) -> Self {
        Self::Repetition(Box::new(self), rep)
    }

    /// make positive lookahead
    pub fn positive_lookahead(peg: Peg) -> Self {
        Self::Lookahead(Box::new(peg), true)
    }

    /// make negative lookahead
    pub fn negative_lookahead(peg: Peg) -> Self {
        Self::Lookahead(Box::new(peg), false)
    }
}

impl GetSpan for Peg {
    fn span(&self) -> Span {
        match self {
            Peg::Eoi(eoi) => eoi.span(),
            Peg::Any(any) => any.span(),
            Peg::Literal(literal) => literal.span(),
            Peg::Class(class) => class.span(),
            Peg::RefRule(ref_rule) => ref_rule.span(),
            Peg::Sequence(pegs) | Peg::Choice(pegs) => {
                Span::from_iter(pegs.iter().map(|peg| peg.span()))
            }
            Peg::Repetition(peg, repeat) => peg.span().union(repeat.span()),
            Peg::Lookahead(peg, _) => peg.span(),
        }
    }
}

impl PrettyPrecedence for Peg {
    /// Lower number = tighter binding
    ///
    /// 0 atom
    /// 1 unary
    /// 2 sequence
    /// 3 choice
    fn precedence(&self) -> usize {
        match self {
            Peg::Eoi(_) | Peg::Any(_) | Peg::Literal(_) | Peg::Class(_) | Peg::RefRule(_) => 0,
            Peg::Choice(_) => 1,
            Peg::Sequence(_) => 2,
            Peg::Repetition(_, _) => 3,
            Peg::Lookahead(_, _) => 3,
        }
    }

    fn pretty_precedence(&self, min_prec: usize, theme: &Theme) -> Doc<'_> {
        let prec = self.precedence();

        let doc = match self {
            // ---------- atoms (NEVER parenthesize) ----------
            Peg::Eoi(eoi) => eoi.pretty(theme),
            Peg::Any(any) => any.pretty(theme),
            Peg::Literal(lit) => lit.pretty(theme),
            Peg::Class(class) => class.pretty(theme),
            Peg::RefRule(r) => r.pretty(theme),

            // ---------- postfix ----------
            Peg::Repetition(peg, rep) => {
                peg.pretty_precedence(prec, theme).append(rep.pretty(theme))
            }

            // ---------- prefix ----------
            Peg::Lookahead(inner, pos) => Doc::text(if *pos { "&" } else { "!" })
                .append(Doc::space())
                .append(inner.pretty_precedence(prec, theme)),

            // ---------- infix ----------
            Peg::Sequence(pegs) => Doc::intersperse(
                pegs.iter().map(|p| p.pretty_precedence(prec + 1, theme)),
                Doc::space(),
            ),

            Peg::Choice(pegs) => Doc::intersperse(
                pegs.iter().map(|p| p.pretty_precedence(prec + 1, theme)),
                Doc::space().append("/").append(Doc::space()),
            ),
        };

        if prec != 0 && prec < min_prec {
            Doc::text("(").append(doc).append(")")
        } else {
            doc
        }
        .group()
    }
}

#[cfg(test)]
mod test {
    use location::SetSpan as _;

    use super::*;
    #[test]
    fn span() {
        let peg = Peg::Sequence(vec![
            Literal::from("a").with_span(Span::new(1, 1)).into(),
            Class::from('0'..='9').into(),
            RefRule::from("expr").with_span(Span::new(6, 10)).into(),
            Peg::from(Literal::from("b")).repetition(Repeat::optional()),
            Peg::negative_lookahead(Literal::from("c").into()),
            Peg::positive_lookahead(Literal::from("d").with_span(Span::new(10, 11)).into()),
        ]);
        assert_eq!(peg.span(), Span::new(1, 11));
    }

    #[test]
    fn pretty() {
        let peg = Peg::Sequence(vec![
            Peg::any(),
            Peg::Choice(vec![
                Literal::from("a").into(),
                Class::from('0'..='9').into(),
            ]),
            RefRule::from("expr").into(),
            Peg::from(Literal::from("b")).repetition(Repeat::optional()),
            Peg::negative_lookahead(Literal::from("c").into()),
            Peg::positive_lookahead(Peg::Sequence(vec![
                Literal::from("d").with_span(Span::new(10, 11)).into(),
                Literal::from("e").into(),
            ])),
            Peg::eoi(),
        ]);

        let string = peg.make_string(&Theme::default());
        assert_eq!(
            string,
            ". (\"a\" / [0-9]) expr \"b\"? ! \"c\" & (\"d\" \"e\") $"
        );
    }
}
