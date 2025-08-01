use super::{expression, operator, Expression, ExpressionDefinition, TypeDefinition};
use crate::lexer::meta::Meta;
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ============================================================================
// Definition Keyword
// ============================================================================
#[derive(Debug)]
pub enum DefinitionKeywordT {
    Def,
    Definition,
}
pub type DefinitionKeyword = Meta<DefinitionKeywordT>;

impl Pretty for DefinitionKeywordT {
    fn pretty(&self, theme: &Theme) -> Doc {
        match self {
            DefinitionKeywordT::Def => theme.keyword(&"Def"),
            DefinitionKeywordT::Definition => theme.keyword(&"Definition"),
        }
    }
}

// ============================================================================
// Type Keyword
// ============================================================================
#[derive(Debug)]
pub enum TypeKeywordT {
    Type,
    Ty,
}
pub type TypeKeyword = Meta<TypeKeywordT>;

impl Pretty for TypeKeywordT {
    fn pretty(&self, theme: &Theme) -> Doc {
        match self {
            TypeKeywordT::Type => theme.keyword(&"Type"),
            TypeKeywordT::Ty => theme.keyword(&"Ty"),
        }
    }
}

// ============================================================================
// Eval Keyword
// ============================================================================
#[derive(Debug)]
pub enum EvalKeywordT {
    Eval,
    EvalOp,
}
pub type EvalKeyword = Meta<EvalKeywordT>;

impl Pretty for EvalKeywordT {
    fn pretty(&self, theme: &Theme) -> Doc {
        match self {
            EvalKeywordT::Eval => theme.keyword(&"Eval"),
            EvalKeywordT::EvalOp => theme.operator(&"$"),
        }
    }
}
// ============================================================================
// TypeOf Keyword
// ============================================================================
#[derive(Debug)]
pub enum TypeOfKeywordT {
    TypeOf,
    TypeOfOp,
}
pub type TypeOfKeyword = Meta<EvalKeywordT>;

impl Pretty for TypeOfKeywordT {
    fn pretty(&self, theme: &Theme) -> Doc {
        match self {
            TypeOfKeywordT::TypeOf => theme.keyword(&"TypeOf"),
            TypeOfKeywordT::TypeOfOp => theme.operator(&"?:"),
        }
    }
}

// ============================================================================
// Set Unset Keyword
// ============================================================================
#[derive(Debug)]
pub enum SetKeywordT {
    Set,
    Unset,
}
pub type SetKeyword = Meta<EvalKeywordT>;

impl Pretty for SetKeywordT {
    fn pretty(&self, theme: &Theme) -> Doc {
        match self {
            SetKeywordT::Set => theme.keyword(&"Set"),
            SetKeywordT::Unset => theme.operator(&"?:"),
        }
    }
}

// ============================================================================
// Command Kind
// ============================================================================

#[derive(Debug)]
pub enum CommandKind {
    ExpressionDefinition {
        keyword: DefinitionKeyword,
        def: ExpressionDefinition,
    },
    TypeDefinition {
        keyword: TypeKeyword,
        def: TypeDefinition,
    },
    Eval {
        keyword: EvalKeyword,
        expr: Expression,
    },
    TypeOf {
        keyword: TypeOfKeyword,
        expr: Expression,
    },
    Set {
        keyword: SetKeyword,
        var: expression::Variable,
    },
    UnSet {
        keyword: SetKeyword,
        var: expression::Variable,
    },
}

impl Located for CommandKind {
    fn loc(&self) -> Location {
        match self {
            CommandKind::ExpressionDefinition { keyword, def } => keyword.loc().union(def.loc()),
            CommandKind::TypeDefinition { keyword, def } => keyword.loc().union(def.loc()),
            CommandKind::Eval { keyword, expr } => keyword.loc().union(expr.loc()),
            CommandKind::TypeOf { keyword, expr } => keyword.loc().union(expr.loc()),
            CommandKind::Set { keyword, var, .. } => keyword.loc().union(var.loc()),
            CommandKind::UnSet { keyword, var, .. } => keyword.loc().union(var.loc()),
        }
    }
}

impl Pretty for CommandKind {
    fn pretty(&self, theme: &Theme) -> Doc {
        match self {
            CommandKind::ExpressionDefinition { keyword, def } => Doc::nil()
                .append(keyword.pretty(theme))
                .append(def.pretty(theme)),
            CommandKind::TypeDefinition { keyword, def } => Doc::nil()
                .append(keyword.pretty(theme))
                .append(def.pretty(theme)),
            CommandKind::Eval { keyword, expr } => Doc::nil()
                .append(keyword.pretty(theme))
                .append(expr.pretty(theme)),
            CommandKind::TypeOf { keyword, expr } => Doc::nil()
                .append(keyword.pretty(theme))
                .append(expr.pretty(theme)),
            CommandKind::Set { keyword, var } => Doc::nil()
                .append(keyword.pretty(theme))
                .append(var.pretty(theme)),
            CommandKind::UnSet { keyword, var } => Doc::nil()
                .append(keyword.pretty(theme))
                .append(var.pretty(theme)),
        }
    }
}

// ============================================================================
// Command
// ============================================================================
#[derive(Debug)]
pub struct Command {
    pub kind: CommandKind,
    pub dot: operator::Dot,
}

impl Located for Command {
    fn loc(&self) -> Location {
        self.kind.loc().union(self.dot.loc())
    }
}

impl Pretty for Command {
    fn pretty(&self, theme: &Theme) -> Doc {
        Doc::nil()
            .append(self.kind.pretty(theme))
            .append(self.dot.pretty(theme))
    }
}
