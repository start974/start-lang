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
    leading: Infos,

    /// Trailing comments, docs, or blank lines after this node
    trailing: Infos,
}

impl From<CstKind> for Cst {
    fn from(kind: CstKind) -> Self {
        Self {
            kind,
            leading: Infos::default(),
            trailing: Infos::default(),
        }
    }
}

impl Cst {
    /// Add leading information (comments, docs, blank lines) to this CST node
    pub fn with_leading(mut self, info: Info) -> Self {
        self.leading = self.leading.append(info);
        self
    }

    /// Add trailing information (comments, docs, blank lines) to this CST node
    pub fn with_trailing(mut self, info: Info) -> Self {
        self.trailing = self.trailing.append(info);
        self
    }
}

impl Pretty for Cst {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::concat(vec![
            self.leading.pretty(theme),
            self.kind.pretty(theme),
            self.trailing.pretty(theme),
        ])
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
                })
                .with_leading(Info::Comment {
                    is_doc: false,
                    content: "This is a comment".split(" ").map(String::from).collect(),
                })
                .with_leading(Info::Spaces),
                Cst::from(CstKind::Token {
                    content: "token2".to_string(),
                    span: Span::default(),
                })
                .with_leading(Info::Spaces),
                Cst::from(CstKind::Token {
                    content: "token3".to_string(),
                    span: Span::default(),
                }),
            ],
        })
        .with_trailing(Info::Lines);

        let theme = Theme::default();
        let str = cst.make_string(&theme);
        assert_eq!(str, "(* This is a comment *) token1 token2token3\n\n");
    }
}
