use crate::tir::Command;
use pp::pretty::*;

// ============================================================================
// File
// ============================================================================

/// typed commands of a file, in source order
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct File {
    pub commands: Vec<Command>,
}

impl Pretty for File {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.commands.iter().map(|cmd| cmd.pretty(theme)),
            Doc::hardline(),
        )
    }
}
