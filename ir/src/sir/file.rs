use super::Command;

// ============================================================================
// File
// ============================================================================
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct File {
    commands: Vec<Command>,
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
}

impl FromIterator<Command> for File {
    fn from_iter<I: IntoIterator<Item = Command>>(iter: I) -> Self {
        Self {
            commands: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sir::{CommandKind, Node};
    use location::{Span, Spanned as _};

    #[test]
    fn file_keeps_command_order() {
        let help = |name: &str, start| {
            Command::new(
                CommandKind::Help(Node::new(name.to_string(), Span::new(start, start + 1))),
                Span::new(start - 2, start + 2),
            )
        };
        let mut file: File = vec![help("a", 2)].into_iter().collect();
        file.add_command(help("b", 7));

        let names: Vec<_> = file
            .commands()
            .iter()
            .map(|cmd| match &cmd.kind {
                CommandKind::Help(var) => var.name(),
                _ => unreachable!(),
            })
            .collect();
        assert_eq!(names, ["a", "b"]);
        assert_eq!(file.commands()[1].span(), Span::new(5, 9));
    }
}
