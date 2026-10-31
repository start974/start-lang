use crate::sir::{Command, DefId, DefKind, DefTable, Documentation, Variable};
use location::{SourceId, Span};

// ============================================================================
// File
// ============================================================================

/// commands of a file, with the table of its definitions
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct File {
    commands: Vec<Command>,
    defs: DefTable,
}

impl File {
    /// empty file of `source`
    pub fn new(source: SourceId) -> Self {
        Self {
            commands: Vec::new(),
            defs: DefTable::new(source),
        }
    }

    /// get the file the commands come from
    pub fn source(&self) -> &SourceId {
        self.defs.source()
    }

    /// get commands
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    /// add command to file
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    /// get the definitions
    pub fn defs(&self) -> &DefTable {
        &self.defs
    }

    /// give an identifier to the definition of `var`, whose whole definition
    /// spans `span`; the definition node is then built with this identifier
    pub fn define(
        &mut self,
        var: Variable,
        kind: DefKind,
        span: Span,
        doc: Option<Documentation>,
    ) -> DefId {
        self.defs.define(var, kind, span, doc)
    }

    /// documentation of the last definition named `name` (for `Help`)
    pub fn documentation_of(&self, name: &str) -> Option<&Documentation> {
        self.defs.last(name).and_then(|id| self.defs.doc(id))
    }
}

impl FromIterator<Command> for File {
    fn from_iter<I: IntoIterator<Item = Command>>(iter: I) -> Self {
        Self {
            commands: iter.into_iter().collect(),
            defs: DefTable::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sir::{CommandKind, DefKind, Node};
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

    #[test]
    fn file_defines_definitions() {
        let var = |start| Node::new("a".to_string(), Span::new(start, start + 1));
        let doc = |line: &str| Documentation::from(vec![line.to_string()]);
        let mut file = File::new(SourceId::Repl);
        let a0 = file.define(
            var(4),
            DefKind::Expression,
            Span::new(4, 10),
            Some(doc("first a")),
        );
        let a1 = file.define(var(14), DefKind::Expression, Span::new(14, 20), None);

        assert_eq!(file.source(), &SourceId::Repl);
        assert_eq!(file.defs().len(), 2);
        assert_eq!(file.defs().all("a"), [a0, a1]);
        assert_eq!(file.defs().doc(a0), Some(&doc("first a")));
        // `Help a` looks at the last definition, which has no documentation
        assert_eq!(file.documentation_of("a"), None);

        file.define(
            var(24),
            DefKind::Expression,
            Span::new(24, 30),
            Some(doc("third a")),
        );
        assert_eq!(file.documentation_of("a"), Some(&doc("third a")));
    }
}
