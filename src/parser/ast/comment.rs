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

#[derive(Debug, Clone)]
pub struct WithComments<T> {
    pub value: T,
    pub before: Vec<Comment>,
    pub after: Vec<Comment>,
}

impl<T> From<T> for WithComments<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            before: Vec::new(),
            after: Vec::new(),
        }
    }
}
impl<T> WithComments<T> {
    /// with commments befor
    pub fn with_before(mut self, comments: Vec<Comment>) -> Self {
        self.before = comments;
        self
    }

    /// with comments afer
    pub fn with_after(mut self, comments: Vec<Comment>) -> Self {
        self.after = comments;
        self
    }

    /// get value
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> Pretty for WithComments<T>
where
    T: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let mut doc = Doc::nil();
        for comment in &self.before {
            doc = doc.append(comment.pretty(theme)).append(Doc::line());
        }
        doc = doc.append(self.value.pretty(theme));
        for comment in &self.after {
            doc = doc.append(Doc::line()).append(comment.pretty(theme));
        }
        doc
    }
}
