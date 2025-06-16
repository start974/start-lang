use super::rule_name::RuleName;
use crate::parser::ast::Identifier;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub enum Syntax {
    Literal(String),      // "a"
    Range(char, char),    // [a..z]
    Seq(Vec<Syntax>),     // a b c
    Option(Box<Syntax>),  // a?
    Repeat0(Box<Syntax>), // a*
    Repeat1(Box<Syntax>), // a+
    Not(Box<Syntax>),     // a?
    Group(Box<Syntax>),   // ~a
    Named {
        name: Identifier,
        syntax: Box<Syntax>,
    }, // < x : a>
    RuleRef(RuleName),    // call a rule
}

impl Pretty for Syntax {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Syntax::Literal(s) => Doc::group(
                Doc::nil()
                    .append("\"")
                    .append(theme.constant(s))
                    .append("\""),
            ),
            Syntax::Range(start, end) => Doc::group(
                Doc::nil()
                    .append(Doc::text("["))
                    .append(Doc::text(start.to_string()))
                    .append(Doc::text(".."))
                    .append(Doc::text(end.to_string()))
                    .append(Doc::text("]")),
            ),
            Syntax::Seq(elements) => {
                Doc::intersperse(elements.iter().map(|e| e.pretty(theme)), Doc::space())
            }
            Syntax::Option(inner) => inner.pretty(theme).append(Doc::text("?")),
            Syntax::Repeat0(inner) => inner.pretty(theme).append(Doc::text("*")),
            Syntax::Repeat1(inner) => inner.pretty(theme).append(Doc::text("+")),
            Syntax::Not(inner) => Doc::text("~").append(inner.pretty(theme)),
            Syntax::Group(inner) => Doc::text("(")
                .append(inner.pretty(theme))
                .append(Doc::text(")")),
            Syntax::Named {
                name,
                syntax: inner,
            } => Doc::group(
                Doc::nil()
                    .append(Doc::text("<"))
                    .append(theme.def_var(name))
                    .append(Doc::space())
                    .append(theme.op_typed_by())
                    .append(Doc::space())
                    .append(inner.pretty(theme))
                    .append(Doc::text(">")),
            ),
            Syntax::RuleRef(rule_name) => rule_name.pretty(theme),
        }
    }
}
