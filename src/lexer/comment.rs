use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug, Clone, PartialEq, Eq)]
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
