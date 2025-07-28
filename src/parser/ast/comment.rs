use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug)]
pub struct Comment {
    content: Vec<String>,
}

impl Comment {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.split("\n").map(|s| s.trim().to_string()).collect(),
        }
    }
}

impl Pretty for Comment {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(theme.comment(&"(*"))
            .append(Doc::space())
            .append(
                Doc::intersperse(
                    self.content.iter().map(|txt| theme.comment(txt)),
                    Doc::line(),
                )
                .group(),
            )
            .append(Doc::space())
            .append(theme.comment(&"*)"))
    }
}

pub trait WithComment {
    /// add commment before
    fn add_comment_before(&mut self, comment: Comment);

    /// add commment after
    fn add_comment_after(&mut self, comment: Comment);

    /// with comment before and after
    fn with_comments<I>(mut self, before: I, after: I) -> Self
    where
        I: IntoIterator<Item = Comment>,
        Self: Sized,
    {
        for comment in before {
            self.add_comment_before(comment);
        }
        for comment in after {
            self.add_comment_after(comment);
        }
        self
    }
}
