use crate::lexer::meta::Meta;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ============================================================================
// Keyword
// ============================================================================
#[derive(Debug)]
pub struct KeywordT(String);
pub type EqDef = Meta<KeywordT>;

impl Pretty for KeywordT {
    fn pretty(&self, theme: &Theme) -> Doc {
        theme.keyword(&self.0)
    }
}


