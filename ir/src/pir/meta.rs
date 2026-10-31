use crate::location::{Span, Spanned};
use crate::pir::Comment;
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

    /// get the lines of the documentation comment just before, if any
    pub fn doc_lines(&self) -> Option<&[String]> {
        self.before.last().and_then(|col| {
            if let CommentOrLines::Comment(c) = col {
                c.doc_lines()
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
        self.pretty_value_with_end_line(theme, end_line, self.value.pretty(theme))
    }

    /// pretty the comments before, then `value` as the document of the value
    pub fn pretty_with<'a>(&'a self, theme: &Theme, value: Doc<'a>) -> Doc<'a> {
        self.pretty_value_with_end_line(theme, true, value)
    }

    fn pretty_value_with_end_line<'a>(
        &'a self,
        theme: &Theme,
        end_line: bool,
        value: Doc<'a>,
    ) -> Doc<'a> {
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
            .append(value)
    }
}

/// used by the parser errors to print the expected tokens
impl<T> std::fmt::Display for Meta<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Spanned for Meta<T> {
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
