use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct Documentation {
    /// documentation lines
    doc: Vec<String>,
}

impl From<Vec<String>> for Documentation {
    fn from(doc: Vec<String>) -> Self {
        Self { doc }
    }
}

impl Pretty for Documentation {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(theme.comment(&"(**"))
            .append(Doc::space())
            .append(
                Doc::intersperse(
                    self.doc.iter().map(|txt| theme.comment(txt)),
                    Doc::hardline(),
                )
                .group(),
            )
            .append(Doc::space())
            .append(theme.comment(&"*)"))
            .group()
            .append(Doc::hardline())
    }
}
