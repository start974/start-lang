use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug, Clone)]
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
        Doc::intersperse(
            self.doc.iter().map(|txt| theme.documentation(txt)),
            Doc::hardline(),
        )
        .group()
    }
}
