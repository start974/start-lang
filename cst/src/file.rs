use super::Command;
use crate::Meta;
use pp::pretty::*;

// ============================================================================
// EndOf File
// ============================================================================

#[derive(Debug, Default)]
pub struct EndOfInputT();

pub type EndOfInput = Meta<EndOfInputT>;

impl Pretty for EndOfInputT {
    fn pretty(&self, _: &Theme) -> Doc<'_> {
        Doc::nil()
    }
}

// ============================================================================
// File
// ============================================================================
#[derive(Debug, Default)]
pub struct File {
    commands: Vec<Command>,
    end: Option<EndOfInput>,
}

impl File {
    /// get commands
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    /// add command to file
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    /// set end of file
    pub fn set_end(&mut self, end: EndOfInput) {
        self.end = Some(end);
    }
}

impl Pretty for File {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.commands().iter().map(|cmd| cmd.pretty(theme)),
            Doc::line(),
        )
        .append(match &self.end {
            Some(end) => end.pretty(theme),
            None => Doc::nil(),
        })
    }
}
