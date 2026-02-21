mod info;
mod tree;

pub use info::*;

use location::Span;
use pp::pretty::*;
use tree::*;

use crate::peg::RefRule;

/// Concrete Syntax Tree node
#[derive(Debug, Clone)]
pub struct Cst(Tree);

impl Default for Cst {
    fn default() -> Self {
        Self::nil()
    }
}

impl Cst {
    /// make nil Cst
    pub fn nil() -> Self {
        Self(Tree::Nil {
            leading: Infos::nil(),
        })
    }

    /// make a token
    pub fn token(content: &str, span: Span) -> Self {
        Self(Tree::Token {
            leading: Infos::nil(),
            content: content.into(),
            trailing: Infos::nil(),
            span,
        })
    }

    /// add a name to cst node
    pub fn with_name(self, name: RefRule) -> Self {
        Self(Tree::Named {
            name,
            leading: Infos::nil(),
            cst: Box::new(self),
            trailing: Infos::nil(),
        })
    }

    /// Add a child to this CST node
    pub fn append(self, other: Cst) -> Self {
        use Tree::*;
        let kind = match (self.0, other.0) {
            (Nil { leading: l1 }, Nil { leading: l2 }) => Nil {
                leading: l1.concat(l2),
            },
            (
                Nil { leading: l1 },
                Token {
                    leading: l2,
                    content,
                    trailing,
                    span,
                },
            ) => Token {
                leading: l1.concat(l2),
                content,
                trailing,
                span,
            },
            (
                Nil { leading: l1 },
                Named {
                    name,
                    leading: l2,
                    cst,
                    trailing,
                },
            ) => Named {
                name,
                leading: l1.concat(l2),
                cst,
                trailing,
            },
            (nil @ Nil { .. }, Node(mut csts)) => {
                let first = csts.first_mut().expect("Nodes cannot be empty");
                *first = Self(nil).append(std::mem::take(first)); // add nil leading
                Node(csts)
            }
            (
                Token {
                    leading,
                    content,
                    trailing: t1,
                    span,
                },
                Nil { leading: t2 },
            ) => Token {
                leading,
                content,
                trailing: t1.concat(t2),
                span,
            },
            (
                Token {
                    leading: l1,
                    content: c1,
                    trailing: l2_1,
                    span: s1,
                },
                Token {
                    leading: l2_2,
                    content: c2,
                    trailing: t2,
                    span: s2,
                },
            ) => Node(vec![
                Self(Token {
                    leading: l1,
                    content: c1,
                    trailing: Infos::nil(),
                    span: s1,
                }),
                Self(Token {
                    leading: l2_1.concat(l2_2),
                    content: c2,
                    trailing: t2,
                    span: s2,
                }),
            ]),
            (
                Token {
                    leading: l1,
                    content,
                    trailing: l2_1,
                    span,
                },
                Named {
                    name,
                    leading: l2_2,
                    cst,
                    trailing: t2,
                },
            ) => Node(vec![
                Self(Token {
                    leading: l1,
                    content,
                    trailing: Infos::nil(),
                    span,
                }),
                Self(Named {
                    name,
                    leading: l2_1.concat(l2_2),
                    cst,
                    trailing: t2,
                }),
            ]),
            (
                Token {
                    leading,
                    content,
                    trailing: l2,
                    span,
                },
                node @ Node(_),
            ) => Node(vec![
                Self(Token {
                    leading,
                    content,
                    trailing: Infos::nil(),
                    span,
                }),
                Self(Nil { leading: l2 }).append(Self(node)),
            ]),
            (
                Named {
                    name,
                    leading,
                    cst,
                    trailing: t1,
                },
                Nil { leading: t2 },
            ) => Named {
                name,
                leading,
                cst,
                trailing: t1.concat(t2),
            },
            (
                Named {
                    name,
                    leading: l1,
                    cst,
                    trailing: l2_1,
                },
                Token {
                    leading: l2_2,
                    content,
                    trailing: t2,
                    span,
                },
            ) => Node(vec![
                Self(Named {
                    name,
                    leading: l1,
                    cst,
                    trailing: Infos::nil(),
                }),
                Self(Token {
                    leading: l2_1.concat(l2_2),
                    content,
                    trailing: t2,
                    span,
                }),
            ]),
            (
                Named {
                    name: n1,
                    leading: l1,
                    cst: c1,
                    trailing: l2_1,
                },
                Named {
                    name: n2,
                    leading: l2_2,
                    cst: c2,
                    trailing: t2,
                },
            ) => Node(vec![
                Self(Named {
                    name: n1,
                    leading: l1,
                    cst: c1,
                    trailing: Infos::nil(),
                }),
                Self(Named {
                    name: n2,
                    leading: l2_1.concat(l2_2),
                    cst: c2,
                    trailing: t2,
                }),
            ]),
            (
                Named {
                    name,
                    leading,
                    cst,
                    trailing: l2,
                },
                node @ Node(_),
            ) => Node(vec![
                Self(Named {
                    name,
                    leading,
                    cst,
                    trailing: Infos::nil(),
                }),
                Self(Nil { leading: l2 }).append(Self(node)),
            ]),
            (Node(mut csts), k2) => {
                let last = csts.pop().expect("Nodes cannot be empty");
                // flatten add
                match last.append(Self(k2)).0 {
                    Node(csts2) => csts.extend(csts2),
                    kind => csts.push(Self(kind)),
                }
                Node(csts)
            }
        };
        Self(kind)
    }
    /// Add leading information (comments, docs, blank lines) to this CST node
    pub fn add_info(self, info: Info) -> Self {
        self.append(Self(Tree::Nil {
            leading: info.into(),
        }))
    }
}

impl Pretty for Cst {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.0.pretty(theme)
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
            .append(Cst::token("token1", Span::default()).with_name("test1".into()))
            .add_info(Info::Spaces)
            .append(Cst::token("token2", Span::default()))
            .add_info(Info::Spaces)
            .append(Cst::token("token3", Span::default()))
            .append(Cst::token("token4", Span::default()))
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
