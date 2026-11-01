use pp::pretty::*;

// ============================================================================
// Eq Def
// ============================================================================
#[derive(Debug, Clone)]
pub struct EqDef;

impl Pretty for EqDef {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&":=")
    }
}

// ============================================================================
// Colon
// ============================================================================
#[derive(Debug, Clone)]
pub struct Colon;

impl Pretty for Colon {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&":")
    }
}

// ============================================================================
// LParen
// ============================================================================
#[derive(Debug, Clone)]
pub struct LParen;

impl Pretty for LParen {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&"(")
    }
}

// ============================================================================
// RParent
// ============================================================================
#[derive(Debug, Clone)]
pub struct RParen;

impl Pretty for RParen {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&")")
    }
}

// ============================================================================
// Dot
// ============================================================================
#[derive(Debug, Clone)]
pub struct Dot;

impl Pretty for Dot {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        theme.operator(&".")
    }
}
