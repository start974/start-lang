use crate::lexer::meta::Meta;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ============================================================================
// Eq Def
// ============================================================================
#[derive(Debug)]
pub struct EqDefT();
pub type EqDef = Meta<EqDefT>;

impl Pretty for EqDefT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.operator(&":=")
    }
}

// ============================================================================
// Colon
// ============================================================================
#[derive(Debug)]
pub struct ColonT();
pub type Colon = Meta<ColonT>;

impl Pretty for ColonT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.operator(&":")
    }
}

// ============================================================================
// LParen
// ============================================================================
#[derive(Debug)]
pub struct LParenT();
pub type LParen = Meta<LParenT>;

impl Pretty for LParenT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.operator(&"(")
    }
}

// ============================================================================
// RParent
// ============================================================================
#[derive(Debug)]
pub struct RParenT();
pub type RParen = Meta<RParenT>;

impl Pretty for RParenT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.operator(&"(")
    }
}

// ============================================================================
// Dot
// ============================================================================
#[derive(Debug)]
pub struct DotT();
pub type Dot = Meta<DotT>;

impl Pretty for DotT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.operator(&".")
    }
}

