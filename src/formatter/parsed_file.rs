use crate::parser::ast::Command;
use crate::utils::location::SourceId;
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

pub struct ParsedFile {
    source_id: SourceId,
    content: Vec<Command>,
}

impl ParsedFile {
    pub fn new(source_id: SourceId) -> Self {
        Self {
            source_id,
            content: Vec::new(),
        }
    }

    /// content of file
    pub fn content(&self) -> &[Command] {
        &self.content
    }

    /// add command to file
    pub fn add_command(&mut self, command: Command) {
        self.content.push(command);
    }

    /// get source id
    pub fn source_id(&self) -> &SourceId {
        &self.source_id
    }
}

impl Pretty for ParsedFile {
    fn pretty(&self, theme: &Theme) -> Doc {
        Doc::intersperse(
            self.content()
                .iter()
                .map(|cmd| cmd.pretty(theme))
                .collect::<Vec<_>>(),
            Doc::line().append(Doc::line()),
        )
    }
}
