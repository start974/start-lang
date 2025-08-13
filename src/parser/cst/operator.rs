use crate::lexer::meta::Meta;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ============================================================================
// Eq Def
// ============================================================================
#[derive(Debug, Clone)]
pub struct EqDefT();
pub type EqDef = Meta<EqDefT>;

impl Pretty for EqDefT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&":=")
    }
}

// ============================================================================
// Colon
// ============================================================================
#[derive(Debug, Clone)]
pub struct ColonT();
pub type Colon = Meta<ColonT>;

impl Pretty for ColonT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&":")
    }
}

// ============================================================================
// LParen
// ============================================================================
#[derive(Debug, Clone)]
pub struct LParenT();
//pub type LParen = Meta<LParenT>;

impl Pretty for LParenT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&"(")
    }
}

// ============================================================================
// RParent
// ============================================================================
#[derive(Debug, Clone)]
pub struct RParenT();
//pub type RParen = Meta<RParenT>;

impl Pretty for RParenT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&")")
    }
}

// ============================================================================
// Dot
// ============================================================================
#[derive(Debug, Clone)]
pub struct DotT();
pub type Dot = Meta<DotT>;

impl Pretty for DotT {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&".")
    }
}
