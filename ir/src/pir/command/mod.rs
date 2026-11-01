use super::{Expression, ExpressionDefinition, TypeDefinition, Variable, operator};
use crate::location::{Span, Spanned};
use crate::token::Token;
use pp::pretty::*;

pub mod keyword;

use keyword::*;

// ============================================================================
// Command Kind
// ============================================================================

#[derive(Debug)]
pub enum CommandKind {
    ExpressionDefinition {
        keyword: Token<DefinitionKeyword>,
        def: Box<ExpressionDefinition>,
    },
    TypeDefinition {
        keyword: Token<TypeKeyword>,
        def: TypeDefinition,
    },
    Eval {
        keyword: Token<EvalKeyword>,
        expr: Expression,
    },
    TypeOf {
        keyword: Token<TypeOfKeyword>,
        expr: Expression,
    },
    Help {
        keyword: Token<HelpKeyword>,
        var: Variable,
    },
    Set {
        keyword: Token<SetKeyword>,
        var: Variable,
    },
    UnSet {
        keyword: Token<UnsetKeyword>,
        var: Variable,
    },
}

impl Pretty for CommandKind {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let doc_keyword = match self {
            CommandKind::ExpressionDefinition { keyword, .. } => keyword.pretty(theme),
            CommandKind::TypeDefinition { keyword, .. } => keyword.pretty(theme),
            CommandKind::Eval { keyword, .. } => keyword.pretty(theme),
            CommandKind::TypeOf { keyword, .. } => keyword.pretty(theme),
            CommandKind::Help { keyword, .. } => keyword.pretty(theme),
            CommandKind::Set { keyword, .. } => keyword.pretty(theme),
            CommandKind::UnSet { keyword, .. } => keyword.pretty(theme),
        };
        let doc_content = match self {
            CommandKind::ExpressionDefinition { def, .. } => def.pretty(theme),
            CommandKind::TypeDefinition { def, .. } => def.pretty(theme),
            CommandKind::Eval { expr, .. } => expr.pretty(theme),
            CommandKind::TypeOf { expr, .. } => expr.pretty(theme),
            CommandKind::Help { var, .. }
            | CommandKind::Set { var, .. }
            | CommandKind::UnSet { var, .. } => var.pretty_with(theme, theme.expr_var(&var.value)),
        };

        Doc::nil()
            .append(doc_keyword)
            .append(Doc::softline())
            .append(doc_content.nest(2))
    }
}

impl Spanned for CommandKind {
    fn span(&self) -> Span {
        let span_keyword = match self {
            CommandKind::ExpressionDefinition { keyword, .. } => keyword.span(),
            CommandKind::TypeDefinition { keyword, .. } => keyword.span(),
            CommandKind::Eval { keyword, .. } => keyword.span(),
            CommandKind::TypeOf { keyword, .. } => keyword.span(),
            CommandKind::Help { keyword, .. } => keyword.span(),
            CommandKind::Set { keyword, .. } => keyword.span(),
            CommandKind::UnSet { keyword, .. } => keyword.span(),
        };
        let span_content = match self {
            CommandKind::ExpressionDefinition { def, .. } => def.span(),
            CommandKind::TypeDefinition { def, .. } => def.span(),
            CommandKind::Eval { expr, .. } => expr.span(),
            CommandKind::TypeOf { expr, .. } => expr.span(),
            CommandKind::Help { var, .. } => var.span(),
            CommandKind::Set { var, .. } => var.span(),
            CommandKind::UnSet { var, .. } => var.span(),
        };
        span_keyword.union(span_content)
    }
}

// ============================================================================
// Command
// ============================================================================
#[derive(Debug)]
pub struct Command {
    pub kind: CommandKind,
    pub dot: Token<operator::Dot>,
}

impl Pretty for Command {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::nil()
            .append(self.kind.pretty(theme))
            .append(if self.dot.has_comment() {
                Doc::line()
            } else {
                Doc::nil()
            })
            .append(self.dot.pretty_with_end_line(theme, false))
            .group()
    }
}

impl Spanned for Command {
    fn span(&self) -> Span {
        self.kind.span().union(self.dot.span())
    }
}
