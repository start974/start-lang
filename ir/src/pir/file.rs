use super::Command;
use crate::token::Token;
use pp::pretty::*;

// ============================================================================
// EndOf File
// ============================================================================

#[derive(Debug, Default)]
pub struct EndOfInput;

impl Pretty for EndOfInput {
    fn pretty(&self, _: &Theme) -> Doc<'_> {
        Doc::nil()
    }
}

// ============================================================================
// File
// ============================================================================
/// commands of a file, and the comments before its end; empty by default
#[derive(Debug, Default)]
pub struct File {
    pub commands: Vec<Command>,
    /// `None` when the file could not be parsed up to its end
    pub end: Option<Token<EndOfInput>>,
}

impl Pretty for File {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.commands.iter().map(|cmd| cmd.pretty(theme)),
            Doc::line(),
        )
        .append(match &self.end {
            Some(end) => end.pretty(theme),
            None => Doc::nil(),
        })
    }
}
