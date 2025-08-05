use super::comment::Comment;
use crate::parser::cst::{AsCharacter, AsIdentifier, AsNumber};
use crate::typing::ast::Documentation;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentOrLines {
    Comment(Comment),
    Lines,
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
    pub fn pretty_with_end_line(&self, theme: &Theme, end_line: bool) -> Doc
    where
        T: Pretty,
    {
        self.pretty_meta(theme)
            .append(match self.before.last() {
                Some(CommentOrLines::Comment(_)) => {
                    if end_line {
                        Doc::line()
                    } else {
                        Doc::line_()
                    }
                }
                _ => Doc::nil(),
            })
            .append(self.value.pretty(theme))
    }

    /// get documentation content on meta before
    pub fn get_doc(&self) -> Option<Documentation> {
        if let Some(CommentOrLines::Comment(comment)) = self.before.last() {
            comment.get_doc()
        } else {
            None
        }
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
