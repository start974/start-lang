use crate::{AsCharacter, AsIdentifier, AsNumber};
use crate::{Comment, Documentation};
use location::{Span, GetSpan};
use pp::pretty::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentOrLines {
    Comment(Comment),
    Lines,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta<T> {
    before: Vec<CommentOrLines>,
    pub value: T,
    span: Span,
}

impl<T> Meta<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self {
            before: Vec::new(),
            value,
            span,
        }
    }

    /// add comment before
    pub fn add_comment(&mut self, comment: Comment) {
        if self.before.len() == 1 && matches!(self.before.last(), Some(CommentOrLines::Lines)) {
            let _ = self.before.pop();
        }
        self.before.push(CommentOrLines::Comment(comment));
    }

    /// add lines before
    pub fn add_lines(&mut self) {
        if !matches!(self.before.last(), Some(CommentOrLines::Lines)) {
            self.before.push(CommentOrLines::Lines);
        }
    }
    /// with comments or lines items before
    pub fn with_items(mut self, before: &[CommentOrLines]) -> Self {
        for item in before {
            match item {
                CommentOrLines::Comment(comment) => {
                    self.add_comment(comment.clone());
                }
                CommentOrLines::Lines => {
                    self.add_lines();
                }
            }
        }
        self
    }

    /// has comment
    pub fn has_comment(&self) -> bool {
        self.before
            .iter()
            .any(|item| matches!(item, CommentOrLines::Comment(_)))
    }

    /// get doctumentation if exist
    pub fn get_doc(&self) -> Option<Documentation> {
        self.before.last().and_then(|col| {
            if let CommentOrLines::Comment(c) = col {
                c.to_doc()
            } else {
                None
            }
        })
    }

    /// map value
    pub fn map<U, F>(self, f: F) -> Meta<U>
    where
        F: FnOnce(T) -> U,
    {
        Meta {
            value: f(self.value),
            before: self.before,
            span: self.span,
        }
    }

    /// just pretty meta
    pub fn pretty_meta(&self, theme: &Theme) -> Doc<'_> {
        let mut last_is_comment = false;
        let mut doc = Doc::nil();
        for val in self.before.iter() {
            if last_is_comment {
                doc = doc.append(Doc::hardline());
            }
            match val {
                CommentOrLines::Comment(comment) => {
                    doc = doc.append(comment.pretty(theme));
                    last_is_comment = true;
                }
                CommentOrLines::Lines => {
                    doc = doc.append(Doc::hardline());
                    last_is_comment = false;
                }
            }
        }
        doc
    }

    /// pretty without line after comment
    pub fn pretty_with_end_line(&self, theme: &Theme, end_line: bool) -> Doc<'_>
    where
        T: Pretty,
    {
        self.pretty_meta(theme)
            .append(match self.before.last() {
                Some(CommentOrLines::Comment(comment)) => {
                    if comment.is_doc() {
                        Doc::hardline()
                    } else if end_line {
                        Doc::line()
                    } else {
                        Doc::line_()
                    }
                }
                _ => Doc::nil(),
            })
            .append(self.value.pretty(theme))
    }
}

impl<T> std::fmt::Display for Meta<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> GetSpan for Meta<T> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Pretty for Meta<T>
where
    T: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.pretty_with_end_line(theme, true)
    }
}

impl<T> AsIdentifier for Meta<T>
where
    T: AsIdentifier,
{
    fn name(&self) -> &str {
        self.value.name()
    }
}

impl<T> AsNumber for Meta<T>
where
    T: AsNumber,
{
    fn as_number(&self) -> &num_bigint::BigUint {
        self.value.as_number()
    }
}

impl<T> AsCharacter for Meta<T>
where
    T: AsCharacter,
{
    fn as_character(&self) -> char {
        self.value.as_character()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use location::Span;

    #[test]
    fn meta_comment_and_lines() {
        let mut meta = Meta::new(42, Span::new(0, 2));
        let comment = Comment::from("This is a comment");
        meta.add_comment(comment.clone());
        assert!(meta.has_comment());
        assert_eq!(meta.before.len(), 1);

        meta.add_lines();
        assert_eq!(meta.before.len(), 2);

        // Adding another lines should not duplicate
        meta.add_lines();
        assert_eq!(meta.before.len(), 2);

        meta.add_comment(comment);
        assert_eq!(meta.before.len(), 3);
    }

    #[test]
    fn with_items() {
        let items = vec![
            CommentOrLines::Comment(Comment::from("First comment")),
            CommentOrLines::Lines,
            CommentOrLines::Lines,
            CommentOrLines::Comment(Comment::from("Second Doc").with_is_doc(true)),
            CommentOrLines::Lines,
        ];

        let meta = Meta::new(100, Span::new(0, 3)).with_items(&items);
        assert_eq!(meta.before.len(), 4);
        assert!(meta.has_comment());
        assert!(meta.get_doc().is_none());
    }

    #[test]
    fn pretty() {
        struct TestPretty;
        impl Pretty for TestPretty {
            fn pretty(&self, _theme: &Theme) -> Doc<'_> {
                Doc::text("TestPrettyMeta")
            }
        }
        let theme = Theme::default();

        {
            let meta = Meta::new(TestPretty, Span::default()).with_items(&[
                CommentOrLines::Lines,
                CommentOrLines::Comment(Comment::from("First comment")),
                CommentOrLines::Lines,
                CommentOrLines::Lines,
                CommentOrLines::Lines,
                CommentOrLines::Comment(Comment::from("Second Doc").with_is_doc(true)),
            ]);

            assert_eq!(meta.before.len(), 3);
            assert!(meta.get_doc().is_some());
            let theme = Theme::default();
            assert_eq!(
                meta.make_string(&theme).to_string(),
                "(* First comment *)\n\n(** Second Doc *)\nTestPrettyMeta"
            );
        }

        {
            let meta = Meta::new(TestPretty, Span::default()).with_items(&[
                CommentOrLines::Lines,
                CommentOrLines::Comment(Comment::from("First comment")),
            ]);

            assert_eq!(
                meta.make_string(&theme).to_string(),
                "(* First comment *)\nTestPrettyMeta"
            );
        }

        {
            let meta = Meta::new(TestPretty, Span::default());
            assert_eq!(meta.make_string(&theme).to_string(), "TestPrettyMeta");
        }
    }

    #[test]
    fn map_value() {
        let meta = Meta::new(10, Span::new(0, 2));
        let new_meta = meta.map(|v| v * 2);
        assert_eq!(new_meta.value, 20);
        assert_eq!(new_meta.span(), Span::new(0, 2));
    }

    #[test]
    fn as_identifier() {
        struct Ident {
            name: String,
        }
        impl AsIdentifier for Ident {
            fn name(&self) -> &str {
                &self.name
            }
        }

        let ident = Ident {
            name: "myIdent".to_string(),
        };
        let meta = Meta::new(ident, Span::new(0, 7));
        assert_eq!(meta.name(), "myIdent");
    }

    #[test]
    fn as_number() {
        struct Num {
            value: num_bigint::BigUint,
        }
        impl AsNumber for Num {
            fn as_number(&self) -> &num_bigint::BigUint {
                &self.value
            }
        }

        let num = Num {
            value: num_bigint::BigUint::from(123u32),
        };
        let meta = Meta::new(num, Span::new(0, 3));
        assert_eq!(meta.as_number(), &num_bigint::BigUint::from(123u32));
    }

    #[test]
    fn as_character() {
        struct Char {
            value: char,
        }
        impl AsCharacter for Char {
            fn as_character(&self) -> char {
                self.value
            }
        }

        let ch = Char { value: 'A' };
        let meta = Meta::new(ch, Span::new(0, 1));
        assert_eq!(meta.as_character(), 'A');
    }

    #[test]
    fn display_meta() {
        let meta = Meta::new(42, Span::new(0, 2));
        assert_eq!(format!("{}", meta), "42");
    }
}
