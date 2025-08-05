use crate::typer::ast::Documentation;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    content: Vec<String>,
    is_doc: bool,
}

impl Comment {
    /// set is doc comment
    pub fn with_is_doc(mut self, is_doc: bool) -> Self {
        self.is_doc = is_doc;
        self
    }

    /// get documentation content
    pub fn get_doc(&self) -> Option<Documentation> {
        if self.is_doc() {
            Some(Documentation::from(self.content.clone()))
        } else {
            None
        }
    }

    /// is doc
    pub fn is_doc(&self) -> bool {
        self.is_doc
    }
}

impl From<String> for Comment {
    fn from(content: String) -> Self {
        let content = content
            .split("\n")
            .map(|s| s.trim().to_string())
            .filter(|s| !String::is_empty(s))
            .collect();
        Self {
            content,
            is_doc: false,
        }
    }
}

impl Pretty for Comment {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let start_doc = if self.is_doc {
            theme.comment(&"(**")
        } else {
            theme.comment(&"(*")
        };

        Doc::nil()
            .append(start_doc)
            .append(Doc::space())
            .append(
                Doc::intersperse(
                    self.content.iter().map(|txt| theme.comment(txt)),
                    Doc::hardline(),
                )
                .group(),
            )
            .append(Doc::space())
            .append(theme.comment(&"*)"))
            .group()
    }
}
