use crate::lir::Global;
use pp::pretty::*;

// ============================================================================
// File
// ============================================================================

/// code of a file: its definitions, a global each, with its type; the vm
/// defines them in order, then evaluates the commands from them
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct File {
    pub globals: Vec<Global>,
}

impl Pretty for File {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.globals.iter().map(|global| global.pretty(theme)),
            Doc::hardline(),
        )
    }
}
