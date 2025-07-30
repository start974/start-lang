use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug, Clone)]
pub struct Comment {
    content: Vec<String>,
}

impl From<String> for Comment {
    fn from(content: String) -> Self {
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

#[derive(Debug, Default)]
pub struct Comments {
    pub before: Vec<Comment>,
    pub after: Vec<Comment>,
}

impl WithComments for Comments {
    /// with comments before
    fn with_comments_before(mut self, comments: Vec<Comment>) -> Self {
        self.before = comments;
        self
    }

    /// with comments after
    fn with_comments_after(mut self, comments: Vec<Comment>) -> Self {
        self.after = comments;
        self
    }

    /// get the comments before
    fn comments_before(&self) -> &[Comment] {
        &self.before
    }

    /// get the comments after
    fn comments_after(&self) -> &[Comment] {
        &self.after
    }
}

pub trait WithComments {
    /// with commments befor
    fn with_comments_before(self, comments: Vec<Comment>) -> Self;

    /// with comments afer
    fn with_comments_after(self, comments: Vec<Comment>) -> Self;

    /// get the comments before
    fn comments_before(&self) -> &[Comment];

    /// get the comments after
    fn comments_after(&self) -> &[Comment];
}

pub trait PrettyWithComments: WithComments {
    type Value: Pretty;

    /// Renvoie la valeur sans les commentaires
    fn value_between_comments(&self) -> &Self::Value;

    fn pretty_with_comments(&self, theme: &Theme) -> Doc<'_> {
        let value = self.value_between_comments();

        let before = Doc::intersperse(
            self.comments_before().iter().map(|c| c.pretty(theme)),
            Doc::line(),
        );

        let after = Doc::intersperse(
            self.comments_after().iter().map(|c| c.pretty(theme)),
            Doc::line(),
        );

        Doc::nil()
            .append(before)
            .append(if self.comments_before().is_empty() {
                Doc::nil()
            } else {
                Doc::line()
            })
            .append(value.pretty(theme))
            .append(if self.comments_after().is_empty() {
                Doc::nil()
            } else {
                Doc::line()
            })
            .append(after)
            .group()
    }
}

impl<T> Pretty for T
where
    T: PrettyWithComments,
    T::Value: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.pretty_with_comments(theme)
    }
}
