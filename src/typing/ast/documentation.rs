use crate::utils::location::{Located, LocatedSet, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct WithDoc<T> {
    doc: Vec<String>,
    pub value: T,
}

impl<T> From<T> for WithDoc<T> {
    fn from(value: T) -> Self {
        Self {
            doc: Vec::new(),
            value,
        }
    }
}

impl<T> WithDoc<T> {
    /// with documentation
    pub fn with_doc(mut self, doc: Vec<String>) -> Self {
        self.doc = doc;
        self
    }
}

impl<T> Pretty for WithDoc<T>
where
    T: Pretty,
{
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let doc = if self.doc.is_empty() {
            Doc::nil()
        } else {
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
        };
        doc.append(self.value.pretty(theme))
    }
}

impl<T> Located for WithDoc<T>
where
    T: Located,
{
    fn loc(&self) -> Location {
        self.value.loc()
    }
}

impl<T> LocatedSet for WithDoc<T>
where
    T: LocatedSet,
{
    fn set_loc(&mut self, loc: &impl Located) {
        self.value.set_loc(loc);
    }
}
