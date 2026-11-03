use crate::location::{Span, Spanned};
use crate::sir::{DefId, Expression, ExpressionDefinition, Node, TypeDefinition};

// ============================================================================
// Command Kind
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandKind {
    ExpressionDefinition(Box<ExpressionDefinition>),
    TypeDefinition(TypeDefinition),
    Eval(Expression),
    TypeOf(Expression),
    /// the definition asked for
    Help(Node<DefId>),
}

// ============================================================================
// Command
// ============================================================================

/// command, with the span of the source it comes from
/// (keyword and final dot included)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub kind: CommandKind,
    pub span: Span,
}

impl Spanned for Command {
    fn span(&self) -> Span {
        self.span
    }
}
