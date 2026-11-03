use crate::sir::{Command, CommandKind, DefId, DefInfo, FileId, Pattern};
use pp::pretty::*;
use std::collections::BTreeMap;

// ============================================================================
// File
// ============================================================================

/// commands of a file, in source order, with the tables of its definitions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub file: FileId,
    pub commands: Vec<Command>,
    /// the definitions of the file, with their documentation, in source order
    pub defs: BTreeMap<DefId, DefInfo>,
}

/// the commands, each definition after its documentation
impl Pretty for File {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        Doc::intersperse(
            self.commands.iter().map(|cmd| {
                let doc = match &cmd.kind {
                    CommandKind::ExpressionDefinition(def) => {
                        let Pattern::Variable(var) = &def.pattern;
                        self.defs[&var.value.id].doc.as_ref()
                    }
                    CommandKind::TypeDefinition(def) => self.defs[&def.id].doc.as_ref(),
                    _ => None,
                };
                match doc {
                    Some(doc) => theme
                        .documentation(&"(**")
                        .append(Doc::space())
                        .append(doc.pretty(theme))
                        .append(Doc::space())
                        .append(theme.documentation(&"*)"))
                        .append(Doc::hardline()),
                    None => Doc::nil(),
                }
                .append(cmd.pretty(theme))
            }),
            Doc::hardline(),
        )
    }
}
