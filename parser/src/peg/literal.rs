use location::{GetSpan, SetSpan, Span};
use pp::pretty::*;

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: String,
    span: Span,
}
impl Literal {
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        Literal {
            value: s.to_string(),
            span: Span::default(),
        }
    }
}

impl GetSpan for Literal {
    fn span(&self) -> Span {
        self.span
    }
}

impl SetSpan for Literal {
    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl Pretty for Literal {
    fn pretty(&self, _theme: &Theme) -> Doc<'_> {
        Doc::text(format!("\"{}\"", self.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let lit = Literal::from("");
        assert!(lit.is_empty());
        assert_eq!(lit.len(), 0);
    }

    #[test]
    fn non_empty() {
        let lit = Literal::from("hello");
        assert!(!lit.is_empty());
        assert_eq!(lit.len(), 5);
    }

    #[test]
    fn span() {
        let mut lit = Literal::from("test");
        assert_eq!(lit.span(), Span::default());
        lit = lit.with_span(Span::new(1, 5));
        assert_eq!(lit.span(), Span::new(1, 5));
    }

    #[test]
    fn pretty() {
        let lit = Literal::from("hello");
        let theme = Theme::default();
        assert_eq!(lit.make_string(&theme), "\"hello\"");
    }
}
