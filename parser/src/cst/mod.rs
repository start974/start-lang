mod info;
mod kind;

pub use info::*;

use kind::*;
use location::Span;
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
        Cst {
            kind,
            leading: Infos::default(),
            trailing: Infos::default(),
        }
    }
}

impl Default for Cst {
    fn default() -> Self {
        Cst {
            kind: CstKind::Node(Vec::new()),
            leading: Infos::default(),
            trailing: Infos::default(),
        }
    }
}

impl Cst {
    /// make nil Cst
    pub fn nil() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        match self.kind {
            CstKind::Node(ref children) => children.is_empty(),
            CstKind::Named(_, ref cst) => cst.is_empty(),
            CstKind::Token(_, _) => false,
        }
    }

    fn is_nil(&self) -> bool {
        self.is_empty()
    }

    /// make a token
    pub fn token(content: &str, span: Span) -> Self {
        CstKind::Token(content.into(), span).into()
    }

    /// Add a child to this CST node
    pub fn add(self, child: Cst) -> Self {
        if child.is_nil() {
            self
        } else {
            match self.kind {
                CstKind::Node(children) if children.is_empty() => Self {
                    leading: self.leading.concat(child.leading),
                    trailing: self.trailing.concat(child.trailing),
                    kind: child.kind,
                },
                CstKind::Node(mut children) => {
                    children.push(child);
                    Self {
                        kind: CstKind::Node(children),
                        ..self
                    }
                }
                CstKind::Named(_, _) | CstKind::Token(_, _) => Self {
                    kind: CstKind::Node(vec![self, child]),
                    leading: Infos::default(),
                    trailing: Infos::default(),
                },
            }
        }
    }

    /// add a name to cst node
    pub fn with_name(self, name: &str) -> Self {
        Self {
            kind: CstKind::Named(name.into(), Box::new(self.kind.into())),
            ..self
        }
    }

    /// Add leading information (comments, docs, blank lines) to this CST node
    pub fn add_info(self, info: Info) -> Self {
        if self.is_empty() {
            Self {
                leading: self.leading.append(info),
                ..self
            }
        } else {
            Self {
                trailing: self.trailing.append(info),
                ..self
            }
        }
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
        let cst = Cst::nil()
            .add_info(Info::Comment {
                is_doc: false,
                content: "This is a comment".split(" ").map(String::from).collect(),
            })
            .add_info(Info::Spaces)
            .add_info(Info::Spaces)
            .add(Cst::token("token1", Span::default()).with_name("test1"))
            .add_info(Info::Spaces)
            .add(Cst::token("token2", Span::default()))
            .add_info(Info::Spaces)
            .add(Cst::token("token3", Span::default()))
            .add_info(Info::Lines)
            .add_info(Info::Lines);

        let theme = Theme::default();
        let str = cst.make_string(&theme);
        assert_eq!(str, "(* This is a comment *) token1 token2token3\n\n");
    }
}
