use crate::sir::{Expression, ExpressionDefinition, TypeDefinition, Variable};
use location::{Span, Spanned};

// ============================================================================
// Command Kind
// ============================================================================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandKind {
    ExpressionDefinition(Box<ExpressionDefinition>),
    TypeDefinition(TypeDefinition),
    Eval(Expression),
    TypeOf(Expression),
    Help(Variable),
    Set(Variable),
    UnSet(Variable),
}

// ============================================================================
// Command
// ============================================================================

/// command, with the span of the source it comes from
/// (keyword and final dot included)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub kind: CommandKind,
    span: Span,
}

impl Command {
    pub fn new(kind: CommandKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Spanned for Command {
    fn span(&self) -> Span {
        self.span
    }
}
