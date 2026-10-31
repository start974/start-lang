use crate::tir::Command;

// ============================================================================
// File
// ============================================================================

/// typed commands of a file, in source order
#[derive(Default)]
pub struct File {
    commands: Vec<Command>,
}

impl File {
    /// get commands
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }
}

impl FromIterator<Command> for File {
    fn from_iter<I: IntoIterator<Item = Command>>(iter: I) -> Self {
        Self {
            commands: iter.into_iter().collect(),
        }
    }
}
