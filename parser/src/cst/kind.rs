use super::Cst;
use location::Span;
use pp::pretty::*;

/// Represents a CST node: either a non-terminal with children, or a token (leaf)
#[derive(Debug, Clone)]
pub enum CstKind {
    /// Non-terminal node with child CSTs
    Node {
        /// Name of the grammar rule for this node
        name: String,
        /// children nodes of this non-terminal, in order
        children: Vec<Cst>,
    },

    /// Leaf token with content and span in input
    Token { content: String, span: Span },
}

impl Pretty for CstKind {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            CstKind::Node { children, .. } => Doc::intersperse(
                children.iter().map(|child| child.pretty(theme)),
                Doc::softline(),
            ),
            CstKind::Token { content, .. } => Doc::text(content),
        }.group()
    }
}
