mod info;
mod kind;

pub use info::*;

use kind::*;
use location::Span;
use pp::pretty::*;

use crate::peg::RefRule;

/// Concrete Syntax Tree node
#[derive(Debug, Clone)]
pub struct Cst {
    /// Leading comments, docs, or blank lines before this node
    leading: Infos,

    /// Kind of node: either a leaf token or a non-terminal node
    kind: Kind,

    /// Trailing comments, docs, or blank lines after this node
    trailing: Infos,
}

impl From<Kind> for Cst {
    fn from(kind: Kind) -> Self {
        Cst {
            kind,
            leading: Infos::default(),
            trailing: Infos::default(),
        }
    }
}

impl Default for Cst {
    fn default() -> Self {
        Self::nil()
    }
}

impl Cst {
    /// make nil Cst
    pub fn nil() -> Self {
        Self {
            kind: Kind::Nil,
            leading: Infos::default(),
            trailing: Infos::default(),
        }
    }

    /// make a token
    pub fn token(content: &str, span: Span) -> Self {
        Kind::Token {
            content: content.into(),
            span,
        }
        .into()
    }

    /// Add a child to this CST node
    pub fn add(self, mut other: Cst) -> Self {
        match (self.kind, other.kind) {
            (Kind::Nil, kind) | (kind, Kind::Nil) => Self {
                kind,
                leading: self.leading.concat(self.trailing).concat(other.leading),
                trailing: other.trailing,
            },
            (Kind::Node(mut children), other_kind) => {
                let mut last = children.pop().expect("Nodes cannot be empty");
                other.leading = other.leading.concat(last.trailing);
                last.trailing = Infos::default();
                children.push(last);
                children.push(Self {
                    kind: other_kind,
                    ..other
                });
                Self {
                    kind: Kind::Node(children),
                    ..self
                }
            }
            (kind_1, kind_2) => Self {
                kind: Kind::Node(vec![
                    Self {
                        kind: kind_1,
                        leading: self.leading,
                        trailing: Infos::default(),
                    },
                    Self {
                        kind: kind_2,
                        leading: self.trailing.concat(other.leading),
                        trailing: other.trailing,
                    },
                ]),
                leading: Infos::default(),
                trailing: Infos::default(),
            },
        }
    }

    /// add a name to cst node
    pub fn with_name(self, name: RefRule) -> Self {
        Self {
            kind: Kind::Named {
                name: name,
                cst: Box::new(self.kind.into()),
            },
            ..self
        }
    }

    /// Add leading information (comments, docs, blank lines) to this CST node
    pub fn add_info(self, info: Info) -> Self {
        match self.kind {
            Kind::Nil => Self {
                leading: self.leading.append(info),
                ..self
            },
            Kind::Node(mut csts) => {
                let mut last = csts.pop().expect("Node cannot be empty");
                last = last.add_info(info);
                csts.push(last);
                Self {
                    kind: Kind::Node(csts),
                    ..self
                }
            }

            _ => Self {
                trailing: self.trailing.append(info),
                ..self
            },
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
            .add(Cst::token("token1", Span::default()).with_name("test1".into()))
            .add_info(Info::Spaces)
            .add(Cst::token("token2", Span::default()))
            .add_info(Info::Spaces)
            .add(Cst::token("token3", Span::default()))
            .add(Cst::token("token4", Span::default()))
            .add_info(Info::Lines)
            .add_info(Info::Lines);

        dbg!(&cst);
        let theme = Theme::default();
        let str = cst.make_string(&theme);
        assert_eq!(
            str,
            "(* This is a comment *) token1 token2 token3token4\n\n"
        );
    }
}
