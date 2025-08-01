use super::comment::Comment;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug)]
enum CommentOrLine {
    Comment(Comment),
    Lines,
}

impl Pretty for CommentOrLine {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            CommentOrLine::Comment(comment) => comment.pretty(theme),
            CommentOrLine::Lines => Doc::line(),
        }
    }
}

#[derive(Debug)]
pub struct Meta<T> {
    before: Vec<CommentOrLine>,
    value: T,
    loc: Location,
}

impl<T> Meta<T> {
    pub fn new(value: T, loc: Location) -> Self {
        Self {
            before: Vec::new(),
            value,
            loc,
        }
    }

    /// add comment before
    pub fn add_comment(&mut self, comment: Comment) {
        self.before.push(CommentOrLine::Comment(comment));
    }

    /// with comment before
    pub fn with_comment(mut self, comment: Comment) -> Self {
        self.add_comment(comment);
        self
    }

    /// add lines before
    pub fn add_lines(&mut self) {
        match self.before.last() {
            Some(CommentOrLine::Lines) => (),
            _ => {
                self.before.push(CommentOrLine::Lines);
            }
        }
    }

    /// with lines before
    pub fn with_lines(mut self) -> Self {
        self.add_lines();
        self
    }

    /// get value
    pub fn value(&self) -> &T {
        &self.value
    }

    /// map value
    pub fn map<U, F>(self, f: F) -> Meta<U>
    where
        F: FnOnce(T) -> U,
    {
        Meta {
            value: f(self.value),
            before: self.before,
            loc: self.loc,
        }
    }

    /// just pretty meta
    pub fn pretty_meta(&self, theme: &Theme) -> Doc {
        Doc::intersperse(
            self.before.iter().map(|item| item.pretty(theme)),
            Doc::line(),
        )
    }
}

impl<T> Located for Meta<T> {
    fn loc(&self) -> Location {
        self.loc.clone()
    }
}

impl<T> Pretty for Meta<T>
where
    T: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc {
        self.pretty_meta(theme)
            .append(if self.before.is_empty() {
                Doc::nil()
            } else {
                Doc::line()
            })
            .append(self.value.pretty(theme))
    }
}
