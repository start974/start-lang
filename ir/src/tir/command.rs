use crate::location::{Span, Spanned};
use crate::tir::{DefInfo, DefKind, Expression, ExpressionDefinition, Type};
use pp::pretty::*;

// ============================================================================
// Command Kind
// ============================================================================

/// typed command; type definitions leave no command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandKind {
    ExpressionDefinition(Box<ExpressionDefinition>),
    Eval(Expression),
    TypeOf(Expression),
    Help(Help),
}

// ============================================================================
// Command
// ============================================================================

/// typed command, with the span of the source it comes from
/// (keyword and final dot included)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub kind: CommandKind,
    pub span: Span,
}

/// the command, as `sir` prints it (debug output)
impl Pretty for Command {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let (keyword, doc) = match &self.kind {
            CommandKind::ExpressionDefinition(def) => ("Definition", def.pretty(theme)),
            CommandKind::Eval(expr) => ("Eval", expr.pretty(theme)),
            CommandKind::TypeOf(expr) => ("TypeOf", expr.pretty(theme)),
            CommandKind::Help(help) => ("Help", theme.expr_var(&help.def.id)),
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

// ============================================================================
// Help
// ============================================================================

/// answer of `Help`: the definition asked for, at the span of the name asked
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Help {
    pub def: DefInfo,
    pub span: Span,
}

impl Spanned for Help {
    fn span(&self) -> Span {
        self.span
    }
}

impl Pretty for Help {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let def = &self.def;
        let doc_ty = match (def.kind, &def.ty) {
            (DefKind::Expression, ty) => Doc::nil()
                .append(theme.operator(&":"))
                .append(Doc::softline())
                .append(ty.pretty(theme).group()),
            (DefKind::Type, Type::Builtin(_)) => theme.comment(&"(builtin)"),
            (DefKind::Type, ty) => Doc::nil()
                .append(theme.operator(&":="))
                .append(Doc::softline())
                .append(ty.pretty(theme).group()),
        };
        let documentation = match &def.doc {
            Some(doc) => Doc::hardline().append(doc.pretty(theme)).nest(2),
            None => Doc::nil(),
        };
        theme
            .expr_var(&def.id.name)
            .append(Doc::space())
            .append(doc_ty)
            .append(documentation)
    }
}
