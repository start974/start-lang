use crate::location::{Span, Spanned};
use crate::sir::{Expression, ExpressionDefinition, Identifier, Node, TypeDefinition};
use pp::pretty::*;

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
    Help(Node<Identifier>),
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

impl Pretty for Command {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let (keyword, doc) = match &self.kind {
            CommandKind::ExpressionDefinition(def) => ("Definition", def.pretty(theme)),
            CommandKind::TypeDefinition(def) => ("Type", def.pretty(theme)),
            CommandKind::Eval(expr) => ("Eval", expr.pretty(theme)),
            CommandKind::TypeOf(expr) => ("TypeOf", expr.pretty(theme)),
            CommandKind::Help(var) => ("Help", theme.expr_var(&var.value)),
        };
        theme
            .keyword(&keyword)
            .append(Doc::space())
            .append(doc)
            .append(theme.operator(&"."))
            .group()
    }
}

impl Spanned for Command {
    fn span(&self) -> Span {
        self.span
    }
}
