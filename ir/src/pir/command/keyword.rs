use crate::pir::Meta;
use pp::pretty::*;

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
