mod info;
mod kind;

pub use info::*;
pub use kind::*;

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
