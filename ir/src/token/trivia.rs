use pp::pretty::*;

// ============================================================================
// Comment
// ============================================================================

/// a comment, `(* … *)`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment(pub Vec<String>);

/// the comment of `text`: its non-empty lines, trimmed
impl From<&str> for Comment {
    fn from(text: &str) -> Self {
        Self(lines(text))
    }
}

impl Pretty for Comment {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        pretty_comment("(*", &self.0, theme)
    }
}

// ============================================================================
// DocComment
// ============================================================================

/// a documentation comment, `(** … *)`, of the definition after it
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocComment(pub Vec<String>);

/// the documentation of `text`: its non-empty lines, trimmed
impl From<&str> for DocComment {
    fn from(text: &str) -> Self {
        Self(lines(text))
    }
}

impl Pretty for DocComment {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        pretty_comment("(**", &self.0, theme)
    }
}

/// non-empty lines of `text`, trimmed
fn lines(text: &str) -> Vec<String> {
    text.split('\n')
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

/// `open` the lines `*)`
fn pretty_comment<'a>(open: &'static str, content: &'a [String], theme: &Theme) -> Doc<'a> {
    Doc::nil()
        .append(theme.comment(&open))
        .append(Doc::space())
        .append(
            Doc::intersperse(
                content.iter().map(|txt| theme.comment(txt)),
                Doc::hardline(),
            )
            .group(),
        )
        .append(Doc::space())
        .append(theme.comment(&"*)"))
        .group()
}

// ============================================================================
// Trivia
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
/// what can be written before a token: a comment, a documentation comment, or
/// blank lines
pub enum Trivia {
    Comment(Comment),
    DocComment(DocComment),
    BlankLines,
}

impl From<Comment> for Trivia {
    fn from(comment: Comment) -> Self {
        Trivia::Comment(comment)
    }
}

impl From<DocComment> for Trivia {
    fn from(doc_comment: DocComment) -> Self {
        Trivia::DocComment(doc_comment)
    }
}
