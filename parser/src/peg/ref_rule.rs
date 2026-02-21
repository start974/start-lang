use location::{GetSpan, SetSpan, Span};
use pp::pretty::*;

#[derive(Debug, Clone)]
pub struct RefRule {
    /// name of the rule being referenced, e.g. `expr` in `expr = other` where `other` is the name of the rule being referenced
    pub name: String,

    /// span of the rule reference, used for error reporting
    span: Span,
}

impl From<&str> for RefRule {
    fn from(s: &str) -> Self {
        RefRule {
            name: s.to_string(),
            span: Span::default(),
        }
    }
}

impl GetSpan for RefRule {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for RefRule {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl Pretty for RefRule {
    fn pretty(&self, _theme: &Theme) -> Doc<'_> {
        Doc::text(&self.name)
    }
}

impl PartialEq for RefRule {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for RefRule {}

impl std::hash::Hash for RefRule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span() {
        let mut r = RefRule::from("expr");
        assert_eq!(r.name, "expr");
        assert_eq!(r.span, Span::default());

        r = r.with_span(Span::new(1, 5));
        assert_eq!(r.span, Span::new(1, 5));
    }
}
