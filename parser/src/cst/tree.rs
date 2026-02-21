use crate::cst::Infos;
use crate::peg::RefRule;

use super::Cst;
use location::Span;
use pp::pretty::*;

/// Represents a CST node: either a non-terminal with children, or a token (leaf)
#[derive(Debug, Clone)]
pub enum Tree {
    /// Leaf nil
    Nil { leading: Infos },

    /// Leaf token with content and span in input
    Token {
        leading: Infos,
        content: String,
        trailing: Infos,
        span: Span,
    },

    /// Non-terminal node with a name and child CSTs
    Named {
        name: RefRule,
        leading: Infos,
        cst: Box<Cst>,
        trailing: Infos,
    },

    /// Non-terminal node with child CSTs
    Node(Vec<Cst>),
}

impl Default for Tree {
    fn default() -> Self {
        Tree::Nil { leading: Infos::default() }
    }
}

impl Pretty for Tree {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            Tree::Nil { leading } => leading.pretty(theme),
            Tree::Token {
                leading,
                content,
                trailing,
                ..
            } => Doc::concat(vec![
                leading.pretty(theme),
                Doc::text(content),
                trailing.pretty(theme),
            ]),
            Tree::Named {
                leading,
                cst,
                trailing,
                ..
            } => Doc::concat(vec![
                leading.pretty(theme),
                cst.pretty(theme),
                trailing.pretty(theme),
            ]),
            Tree::Node(children) => {
                Doc::intersperse(children.iter().map(|child| child.pretty(theme)), Doc::nil())
            }
        }
        .group()
    }
}
