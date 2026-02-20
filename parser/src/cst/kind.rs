use crate::peg::RefRule;

use super::Cst;
use location::Span;
use pp::pretty::*;

/// Represents a CST node: either a non-terminal with children, or a token (leaf)
#[derive(Debug, Clone)]
pub enum CstKind {
    /// Non-terminal node with child CSTs
    Node(Vec<Cst>),

    /// Non-terminal node with a name and child CSTs
    Named { name: RefRule, cst: Box<Cst> },

    /// Leaf token with content and span in input
    Token { content: String, span: Span },
}

impl Pretty for CstKind {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            CstKind::Node(children) => {
                Doc::intersperse(children.iter().map(|child| child.pretty(theme)), Doc::nil())
            }
            CstKind::Named { cst, .. } => cst.pretty(theme),
            CstKind::Token { content, .. } => Doc::text(content),
        }
        .group()
    }
}
