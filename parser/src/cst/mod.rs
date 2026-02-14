mod info;
mod kind;

pub use info::*;
pub use kind::*;

use pp::pretty::*;

/// Concrete Syntax Tree node
#[derive(Debug, Clone)]
pub struct Cst {
    /// Kind of node: either a leaf token or a non-terminal node
    kind: CstKind,

    /// Leading comments, docs, or blank lines before this node
    leading: Vec<Info>,

    /// Trailing comments, docs, or blank lines after this node
    trailing: Vec<Info>,
}

impl From<CstKind> for Cst {
    fn from(kind: CstKind) -> Self {
        Self {
            kind,
            leading: Vec::new(),
            trailing: Vec::new(),
        }
    }
}

impl Cst {
    /// Add leading information (comments, docs, blank lines) to this CST node
    pub fn with_leading(mut self, info: Info) -> Self {
        self.leading.push(info);
        self
    }

    /// Add trailing information (comments, docs, blank lines) to this CST node
    pub fn with_trailing(mut self, info: Info) -> Self {
        self.trailing.push(info);
        self
    }
}

fn pretty_infos<'a>(infos: &'a [Info], theme: &Theme) -> Doc<'a> {
    if infos.is_empty() {
        Doc::nil()
    } else {
        let mut doc = Doc::nil();
        for info in infos {
            doc = doc.append(info.pretty(theme));
            if !info.is_lines() {
                doc = doc.append(Doc::softline());
            }
        }
        doc
    }
}

impl Pretty for Cst {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let leading_doc = pretty_infos(&self.leading, theme);
        let trailing_doc = pretty_infos(&self.trailing, theme);
        Doc::concat(vec![leading_doc, self.kind.pretty(theme), trailing_doc])
    }
}

#[cfg(test)]
mod tests {
    use location::Span;

    use super::*;

    #[test]
    fn pretty_cst() {
        let cst = Cst::from(CstKind::Node {
            name: "Example".to_string(),
            children: vec![
                Cst::from(CstKind::Token {
                    content: "token1".to_string(),
                    span: Span::default(),
                }),
                Cst::from(CstKind::Token {
                    content: "token2".to_string(),
                    span: Span::default(),
                }),
            ],
        })
        .with_leading(Info::Comment {
            is_doc: false,
            content: "This is a comment".split(" ").map(String::from).collect(),
        })
        .with_trailing(Info::Lines);

        let theme = Theme::default();
        let str = cst.make_string(&theme);
        assert_eq!(str, "(* This is a comment *) token1 token2\n\n");
    }
}
