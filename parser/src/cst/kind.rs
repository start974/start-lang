use location::Span;

use super::Cst;

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
