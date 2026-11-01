use pp::pretty::*;

// ============================================================================
// Definition Keyword
// ============================================================================
#[derive(Debug)]
pub enum DefinitionKeyword {
    Def,
    Definition,
}

impl Pretty for DefinitionKeyword {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            DefinitionKeyword::Def => theme.keyword(&"Def"),
            DefinitionKeyword::Definition => theme.keyword(&"Definition"),
        }
    }
}

// ============================================================================
// Type Keyword
// ============================================================================
#[derive(Debug)]
pub enum TypeKeyword {
    Type,
    Ty,
}

impl Pretty for TypeKeyword {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            TypeKeyword::Type => theme.keyword(&"Type"),
            TypeKeyword::Ty => theme.keyword(&"Ty"),
        }
    }
}

// ============================================================================
// Eval Keyword
// ============================================================================
#[derive(Debug)]
pub enum EvalKeyword {
    Eval,
    EvalOp,
}

impl Pretty for EvalKeyword {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            EvalKeyword::Eval => theme.keyword(&"Eval"),
            EvalKeyword::EvalOp => theme.operator(&"$"),
        }
    }
}

// ============================================================================
// TypeOf Keyword
// ============================================================================
#[derive(Debug)]
pub enum TypeOfKeyword {
    TypeOf,
    TypeOfOp,
}

impl Pretty for TypeOfKeyword {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            TypeOfKeyword::TypeOf => theme.keyword(&"TypeOf"),
            TypeOfKeyword::TypeOfOp => theme.operator(&"?:"),
        }
    }
}

// ============================================================================
// Help Keyword
// ============================================================================
#[derive(Debug)]
pub enum HelpKeyword {
    Help,
    HelpOp,
}

impl Pretty for HelpKeyword {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match self {
            HelpKeyword::Help => theme.keyword(&"Help"),
            HelpKeyword::HelpOp => theme.operator(&"?"),
        }
    }
}

// ============================================================================
// Set Keyword
// ============================================================================
#[derive(Debug)]
pub struct SetKeyword;

impl Pretty for SetKeyword {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.keyword(&"Set")
    }
}

// ============================================================================
// Unset Keyword
// ============================================================================
#[derive(Debug)]
pub struct UnsetKeyword;

impl Pretty for UnsetKeyword {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.keyword(&"Unset")
    }
}
