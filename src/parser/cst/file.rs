use super::Command;
use crate::lexer::meta::Meta;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

// ============================================================================
// EndOf File
// ============================================================================

#[derive(Debug, Default)]
pub struct EndOfFileT();

pub type EndOfFile = Meta<EndOfFileT>;

impl Pretty for EndOfFileT {
    fn pretty(&self, _: &Theme) -> Doc {
        Doc::nil()
    }
}

// ============================================================================
// File
// ============================================================================
#[derive(Debug, Default)]
pub struct File {
    commands: Vec<Command>,
    end: Option<EndOfFile>,
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
    pub fn set_end(&mut self, end: EndOfFile) {
        self.end = Some(end);
    }
}

impl Pretty for File {
    fn pretty(&self, theme: &Theme) -> Doc {
        Doc::intersperse(
            self.commands().iter().map(|cmd| cmd.pretty(theme)),
            Doc::nil(),
        )
        .append(match &self.end {
            Some(end) => end.pretty(theme),
            None => Doc::nil(),
        })
    }
}
