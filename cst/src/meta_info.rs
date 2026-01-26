use crate::{Comment, Documentation};
use pp::pretty::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentOrLines {
    Comment(Comment),
    Lines,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MetaInfo {
    before: Vec<CommentOrLines>,
}

impl MetaInfo {
    /// add comment before
    pub fn with_comment(mut self, comment: Comment) -> Self {
        if self.before.len() == 1 && matches!(self.before.last(), Some(CommentOrLines::Lines)) {
            let _ = self.before.pop();
        }
        self.before.push(CommentOrLines::Comment(comment));
        self
    }

    /// add lines before
    pub fn with_lines(mut self) -> Self {
        if !matches!(self.before.last(), Some(CommentOrLines::Lines)) {
            self.before.push(CommentOrLines::Lines);
        }
        self
    }
    /// with comments or lines items before
    pub fn with_items(self, before: &[CommentOrLines]) -> Self {
        before.iter().fold(self, |meta, item| match item {
            CommentOrLines::Comment(comment) => meta.with_comment(comment.clone()),
            CommentOrLines::Lines => meta.with_lines(),
        })
    }

    /// has comment
    pub fn has_comment(&self) -> bool {
        self.before
            .iter()
            .any(|item| matches!(item, CommentOrLines::Comment(_)))
    }

    /// get doctumentation if exist
    pub fn get_doc(&self) -> Option<Documentation> {
        self.before.last().and_then(|col| {
            if let CommentOrLines::Comment(c) = col {
                c.to_doc()
            } else {
                None
            }
        })
    }
}

impl Pretty for MetaInfo {
    /// just pretty meta
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
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
}

pub trait GetMetaInfo {
    /// get meta info
    fn meta_info(&self) -> &MetaInfo;
}

pub trait SetMetaInfo: Sized {
    /// set meta info
    fn set_meta_info(&mut self, meta: MetaInfo);

    /// with meta info
    fn with_meta_info(mut self, meta: MetaInfo) -> Self {
        self.set_meta_info(meta);
        self
    }
}

pub trait PrettyMetaInfo: GetMetaInfo {
    fn pretty_inner(&self, theme: &Theme) -> Doc<'_>;

    /// pretty with line after comment
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.meta_info()
            .pretty(theme)
            .append(match self.meta_info().before.last() {
                Some(CommentOrLines::Comment(comment)) => {
                    if comment.is_doc() {
                        Doc::hardline()
                    } else {
                        Doc::line()
                    }
                }
                _ => Doc::nil(),
            })
            .append(self.pretty_inner(theme))
    }
}

impl FromIterator<CommentOrLines> for MetaInfo {
    fn from_iter<I: IntoIterator<Item = CommentOrLines>>(iter: I) -> Self {
        Self {
            before: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for MetaInfo {
    type Item = CommentOrLines;
    type IntoIter = std::vec::IntoIter<CommentOrLines>;

    fn into_iter(self) -> Self::IntoIter {
        self.before.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_comment_and_lines() {
        let mut meta = MetaInfo::default();
        let comment = Comment::from("This is a comment");
        meta = meta.with_comment(comment.clone());
        assert!(meta.has_comment());
        assert_eq!(meta.before.len(), 1);

        meta = meta.with_lines();
        assert_eq!(meta.before.len(), 2);

        // Adding another lines should not duplicate
        meta = meta.with_lines();
        assert_eq!(meta.before.len(), 2);

        meta = meta.with_comment(comment);
        assert_eq!(meta.before.len(), 3);
    }

    #[test]
    fn with_items() {
        let items = vec![
            CommentOrLines::Comment(Comment::from("First comment")),
            CommentOrLines::Lines,
            CommentOrLines::Lines,
            CommentOrLines::Comment(Comment::from("Second Doc").with_is_doc(true)),
            CommentOrLines::Lines,
        ];

        let meta = MetaInfo::default().with_items(&items);
        assert_eq!(meta.before.len(), 4);
        assert!(meta.has_comment());
        assert!(meta.get_doc().is_none());
    }

    #[test]
    fn pretty() {
        struct TestPretty {
            meta: MetaInfo,
        }

        impl GetMetaInfo for TestPretty {
            fn meta_info(&self) -> &MetaInfo {
                &self.meta
            }
        }

        impl PrettyMetaInfo for TestPretty {
            fn pretty_inner(&self, _theme: &Theme) -> Doc<'_> {
                Doc::text("TestPrettyMeta")
            }
        }

        impl Pretty for TestPretty {
            fn pretty(&self, theme: &Theme) -> Doc<'_> {
                PrettyMetaInfo::pretty(self, theme)
            }
        }
        let theme = Theme::default();

        {
            let meta = MetaInfo::default().with_items(&[
                CommentOrLines::Lines,
                CommentOrLines::Comment(Comment::from("First comment")),
                CommentOrLines::Lines,
                CommentOrLines::Lines,
                CommentOrLines::Lines,
                CommentOrLines::Comment(Comment::from("Second Doc").with_is_doc(true)),
            ]);

            assert_eq!(meta.before.len(), 3);
            assert!(meta.get_doc().is_some());
            let test_pretty = TestPretty { meta };
            let theme = Theme::default();
            assert_eq!(
                test_pretty.make_string(&theme).to_string(),
                "(* First comment *)\n\n(** Second Doc *)\nTestPrettyMeta"
            );
        }

        {
            let meta = MetaInfo::default().with_items(&[
                CommentOrLines::Lines,
                CommentOrLines::Comment(Comment::from("First comment")),
            ]);

            let test_pretty = TestPretty { meta };
            assert_eq!(
                test_pretty.make_string(&theme).to_string(),
                "(* First comment *)\nTestPrettyMeta"
            );
        }

        {
            let meta = MetaInfo::default();
            let test_pretty = TestPretty { meta };
            assert_eq!(
                test_pretty.make_string(&theme).to_string(),
                "TestPrettyMeta"
            );
        }
    }
}
