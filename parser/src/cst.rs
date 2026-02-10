use location::Span;

#[derive(Debug, Clone)]
pub enum Info {
    /// Comment information, e.g. (* comment *)
    Comment(String),
    /// Doc comment information, e.g. (** doc comment *)
    Doc(String),

    /// Information about many lines
    Lines,
}

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

/// Concrete Syntax Tree node
#[derive(Debug, Clone)]
pub struct Cst {
    /// Kind of node: either a leaf token or a non-terminal node
    pub kind: CstKind,

    /// Leading comments, docs, or blank lines before this node
    pub leading: Vec<Info>,

    /// Trailing comments, docs, or blank lines after this node
    pub trailing: Vec<Info>,
}
