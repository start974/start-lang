mod class;
mod literal;
mod ref_rule;
mod repeat;
mod whitespace;

pub use class::*;
pub use literal::*;
pub use ref_rule::*;
pub use repeat::*;
pub use whitespace::*;

use location::{GetSpan, Span};
use pp::pretty::*;

/// Represents a grammar syntax node for PEG parsing and Pratt operators.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Peg {
    /// whitespace and comments, e.g. space, tab, newline.
    /// This is not a token, but a special syntax that
    /// matches any amount of whitespace.
    WS(WhiteSpace),

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
    Repeat(Box<Peg>, Repeat),

    /// Negative lookahead: matches if the inner syntax does NOT match,
    /// without consuming input, e.g. `!a`.
    NegativeLookahead(Box<Peg>),

    /// Positive lookahead: matches if the inner syntax matches,
    /// without consuming input, e.g. `&a`.
    PositiveLookahead(Box<Peg>),
}

impl From<WhiteSpace> for Peg {
    fn from(ws: WhiteSpace) -> Self {
        Peg::WS(ws)
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
    /// is atom
    pub fn is_atom(&self) -> bool {
        matches!(self, Peg::Literal(_) | Peg::Class(_) | Peg::RefRule(_))
    }
}

impl GetSpan for Peg {
    fn span(&self) -> Span {
        match self {
            Peg::WS(ws) => ws.span(),
            Peg::Literal(literal) => literal.span(),
            Peg::Class(class) => class.span(),
            Peg::RefRule(ref_rule) => ref_rule.span(),
            Peg::Seq(pegs) | Peg::Choice(pegs) => Span::from_iter(pegs),
            Peg::Repeat(peg, repeat) => peg.span().union(repeat.span()),
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
            Peg::WS(_) | Peg::Literal(_) | Peg::Class(_) | Peg::RefRule(_) => 0,
            Peg::Choice(_) => 1,
            Peg::Seq(_) => 2,
            Peg::Repeat(_, _) => 3,
            Peg::NegativeLookahead(_) | Peg::PositiveLookahead(_) => 3,
        }
    }

    fn pretty_precedence(&self, min_prec: usize, theme: &Theme) -> Doc<'_> {
        let prec = self.precedence();

        let doc = match self {
            // ---------- atoms (NEVER parenthesize) ----------
            Peg::WS(ws) => ws.pretty(theme),
            Peg::Literal(l) => l.pretty(theme),
            Peg::Class(c) => c.pretty(theme),
            Peg::RefRule(r) => r.pretty(theme),

            // ---------- postfix ----------
            Peg::Repeat(peg, rep) => peg.pretty_precedence(prec, theme).append(rep.pretty(theme)),

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
            Literal::from("a").with_span(Span::new(1, 1)).into(),
            Peg::Class(Class::from('0'..='9')),
            Peg::RefRule(RefRule::from("expr").with_span(Span::new(6, 10))),
            Peg::Repeat(Box::new(Literal::from("b").into()), Repeat::optional()),
            Peg::NegativeLookahead(Box::new(Literal::from("c").into())),
            Peg::PositiveLookahead(Box::new(
                Literal::from("d").with_span(Span::new(10, 11)).into(),
            )),
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
            Peg::Repeat(Box::new(Literal::from("b").into()), Repeat::optional()),
            Peg::NegativeLookahead(Box::new(Literal::from("c").into())),
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
