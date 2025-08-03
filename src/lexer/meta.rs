use super::comment::Comment;
use crate::parser::cst::{AsCharacter, AsIdentifier, AsNumber};
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentOrLines {
    Comment(Comment),
    Lines,
}

impl Pretty for CommentOrLines {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            CommentOrLines::Comment(comment) => comment.pretty(theme).group(),
            CommentOrLines::Lines => Doc::hardline(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta<T> {
    before: Vec<CommentOrLines>,
    pub value: T,
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
        self.before.push(CommentOrLines::Comment(comment));
    }

    /// add lines before
    pub fn add_lines(&mut self) {
        match self.before.last() {
            Some(CommentOrLines::Lines) => (),
            _ => {
                self.before.push(CommentOrLines::Lines);
            }
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
        Doc::concat(self.before.iter().map(|item| item.pretty(theme)))
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
            .append(match self.before.last() {
                Some(CommentOrLines::Comment(_)) => Doc::line(),
                _ => Doc::nil(),
            })
            .append(self.value.pretty(theme))
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
