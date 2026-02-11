mod class;
mod literal;
mod ref_rule;
mod repeat;

pub use class::*;
pub use literal::*;
pub use ref_rule::*;
pub use repeat::*;

use location::{GetSpan, Span};
use pp::pretty::*;

/// Represents a grammar syntax node for PEG parsing and Pratt operators.
#[derive(Debug, Clone)]
pub enum Peg {
    /// A literal string, e.g. 'a', '1', or "abc".
    Literal(Literal),

    /// A character class, e.g. [a-z], [^0-9].
    Class(Class),

    /// Reference to another grammar rule by name.
    RefRule(RefRule),

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

impl Peg {
    /// is atom
    pub fn is_atom(&self) -> bool {
        matches!(self, Peg::Literal(_) | Peg::Class(_) | Peg::RefRule(_))
    }
}

impl GetSpan for Peg {
    fn span(&self) -> Span {
        match self {
            Peg::Literal(literal) => literal.span(),
            Peg::Class(class) => class.span(),
            Peg::RefRule(ref_rule) => ref_rule.span(),
            Peg::Seq(pegs) | Peg::Choice(pegs) => Span::from_iter(pegs),
            Peg::Repeat(repeat) => repeat.span(),
            Peg::NegativeLookahead(peg) | Peg::PositiveLookahead(peg) => peg.span(),
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
            Peg::Literal(_) | Peg::Class(_) | Peg::RefRule(_) => 0,
            Peg::Choice(_) => 1,
            Peg::Seq(_) => 2,
            Peg::Repeat(r) => r.precedence(), // is 3
            Peg::NegativeLookahead(_) | Peg::PositiveLookahead(_) => 3,
        }
    }

    fn pretty_precedence(&self, min_prec: usize, theme: &Theme) -> Doc<'_> {
        let prec = self.precedence();

        let doc = match self {
            // ---------- atoms (NEVER parenthesize) ----------
            Peg::Literal(l) => l.pretty(theme),
            Peg::Class(c) => c.pretty(theme),
            Peg::RefRule(r) => r.pretty(theme),

            // ---------- postfix ----------
            Peg::Repeat(inner) => inner.pretty_precedence(prec, theme),

            // ---------- prefix ----------
            Peg::NegativeLookahead(inner) => Doc::text("!")
                .append(Doc::space())
                .append(inner.pretty_precedence(prec, theme)),

            Peg::PositiveLookahead(inner) => Doc::text("&")
                .append(Doc::space())
                .append(inner.pretty_precedence(prec, theme)),

            // ---------- infix ----------
            Peg::Seq(pegs) => Doc::intersperse(
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
        let peg = Peg::Seq(vec![
            Peg::Literal(Literal::from("a").with_span(Span::new(1, 1))),
            Peg::Class(Class::from('0'..='9')),
            Peg::RefRule(RefRule::from("expr").with_span(Span::new(6, 10))),
            Peg::Repeat(Repeat::optional(Peg::Literal(Literal::from("b")))),
            Peg::NegativeLookahead(Box::new(Peg::Literal(Literal::from("c")))),
            Peg::PositiveLookahead(Box::new(Peg::Literal(
                Literal::from("d").with_span(Span::new(10, 11)),
            ))),
        ]);
        assert_eq!(peg.span(), Span::new(1, 11));
    }

    #[test]
    fn pretty() {
        let peg = Peg::Seq(vec![
            Peg::Choice(vec![
                Peg::Literal(Literal::from("a")),
                Peg::Class(Class::from('0'..='9')),
            ]),
            Peg::RefRule(RefRule::from("expr")),
            Peg::Repeat(Repeat::optional(Peg::Literal(Literal::from("b")))),
            Peg::NegativeLookahead(Box::new(Peg::Literal(Literal::from("c")))),
            Peg::PositiveLookahead(Box::new(Peg::Seq(vec![
                Peg::Literal(Literal::from("d")),
                Peg::Literal(Literal::from("e")),
            ]))),
        ]);

        let string = peg.make_string(&Theme::default());
        assert_eq!(
            string,
            "(\"a\" / [0-9]) expr \"b\"? ! \"c\" & (\"d\" \"e\")"
        );
    }
}
