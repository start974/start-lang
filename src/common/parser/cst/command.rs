use super::{expression, help, operator, Expression, ExpressionDefinition, TypeDefinition};
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
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
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
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
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
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
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
pub type TypeOfKeyword = Meta<TypeOfKeywordT>;

impl Pretty for TypeOfKeywordT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            TypeOfKeywordT::TypeOf => theme.keyword(&"TypeOf"),
            TypeOfKeywordT::TypeOfOp => theme.operator(&"?:"),
        }
    }
}

// ============================================================================
// Help Keyword
// ============================================================================
#[derive(Debug)]
pub enum HelpKeywordT {
    Help,
    HelpOp,
}
pub type HelpKeyword = Meta<HelpKeywordT>;

impl Pretty for HelpKeywordT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            HelpKeywordT::Help => theme.keyword(&"Help"),
            HelpKeywordT::HelpOp => theme.operator(&"?"),
        }
    }
}

// ============================================================================
// Set Keyword
// ============================================================================
#[derive(Debug)]
pub struct SetKeywordT();
pub type SetKeyword = Meta<SetKeywordT>;

impl Pretty for SetKeywordT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.keyword(&"Set")
    }
}

// ============================================================================
// Unset Keyword
// ============================================================================
#[derive(Debug)]
pub struct UnsetKeywordT();
pub type UnsetKeyword = Meta<UnsetKeywordT>;

impl Pretty for UnsetKeywordT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.keyword(&"Unset")
    }
}

// ============================================================================
// Command Kind
// ============================================================================

#[derive(Debug)]
pub enum CommandKind {
    ExpressionDefinition {
        keyword: DefinitionKeyword,
        def: Box<ExpressionDefinition>,
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
    Help {
        keyword: HelpKeyword,
        var: help::Variable,
    },
    Set {
        keyword: SetKeyword,
        var: expression::Variable,
    },
    UnSet {
        keyword: UnsetKeyword,
        var: expression::Variable,
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
            CommandKind::Help { var, .. } => var.pretty(theme),
            CommandKind::Set { var, .. } => var.pretty(theme),
            CommandKind::UnSet { var, .. } => var.pretty(theme),
        };

        Doc::nil()
            .append(doc_keyword)
            .append(Doc::softline())
            .append(doc_content.nest(2))
    }
}

impl Located for CommandKind {
    fn loc(&self) -> Location {
        let loc_keyword = match self {
            CommandKind::ExpressionDefinition { keyword, .. } => keyword.loc(),
            CommandKind::TypeDefinition { keyword, .. } => keyword.loc(),
            CommandKind::Eval { keyword, .. } => keyword.loc(),
            CommandKind::TypeOf { keyword, .. } => keyword.loc(),
            CommandKind::Help { keyword, .. } => keyword.loc(),
            CommandKind::Set { keyword, .. } => keyword.loc(),
            CommandKind::UnSet { keyword, .. } => keyword.loc(),
        };
        let loc_content = match self {
            CommandKind::ExpressionDefinition { def, .. } => def.loc(),
            CommandKind::TypeDefinition { def, .. } => def.loc(),
            CommandKind::Eval { expr, .. } => expr.loc(),
            CommandKind::TypeOf { expr, .. } => expr.loc(),
            CommandKind::Help { var, .. } => var.loc(),
            CommandKind::Set { var, .. } => var.loc(),
            CommandKind::UnSet { var, .. } => var.loc(),
        };
        loc_keyword.union(loc_content)
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

impl Located for Command {
    fn loc(&self) -> Location {
        self.kind.loc().union(self.dot.loc())
    }
}
