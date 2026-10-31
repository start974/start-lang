use crate::location::{Span, Spanned};
use crate::tir::{Expression, ExpressionDefinition, Help};

// ============================================================================
// Command Kind
// ============================================================================

/// typed command; type definitions and options leave no command: they only
/// change the environment or are checked
pub enum CommandKind {
    ExpressionDefinition(ExpressionDefinition),
    Eval(Expression),
    TypeOf(Expression),
    Help(Help),
}

// ============================================================================
// Command
// ============================================================================

/// typed command, with the span of the source it comes from
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
