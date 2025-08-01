use super::{Expression, ExpressionDefinition, TypeDefinition, Variable};
use crate::utils::location::{Located, Location};
use crate::utils::pretty::Pretty;
use crate::utils::theme::{Doc, Theme};

#[derive(Debug)]
pub enum CommandKind {
    ExpressionDefinition(ExpressionDefinition),
    TypeDefinition(TypeDefinition),
    Eval(Expression),
    //Grammar(GrammarCommand),
    TypeOf(Expression),
    Set(bool, Variable),
}

pub struct Command {
    loc: Location,
    pub kind: CommandKind,
}

impl Command {
    /// Create a new command with the given kind and location
    pub fn new(kind: CommandKind, loc: Location) -> Self {
        Self { loc, kind }
    }
}

impl Located for Command {
    fn loc(&self) -> &Location {
        &self.loc
    }
}

impl Pretty for CommandKind {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        match &self {
            CommandKind::ExpressionDefinition(def) => Doc::nil()
                .append(theme.keyword(&"Definition"))
                .append(Doc::space())
                .append(def.pretty(theme)),
            CommandKind::TypeDefinition(def) => Doc::nil()
                .append(theme.keyword(&"Type"))
                .append(Doc::space())
                .append(def.pretty(theme)),
            CommandKind::Eval(expr) => Doc::nil()
                .append(theme.keyword(&"Eval"))
                .append(Doc::space())
                .append(expr.pretty(theme)),
            //Command::Grammar(grammar_cmd) => Doc::nil()
            //.append(theme.keyword(&"Grammar"))
            //.append(Doc::space())
            //.append(grammar_cmd.pretty(theme)),
            CommandKind::TypeOf(expr) => Doc::nil()
                .append(theme.keyword(&"TypeOf"))
                .append(Doc::space())
                .append(expr.pretty(theme)),
            CommandKind::Set(set, var) => Doc::nil()
                .append(theme.keyword(&if *set { "Set" } else { "Unset" }))
                .append(Doc::space())
                .append(var.pretty(theme)),
        }
        .append(Doc::text("."))
    }
}

impl Pretty for Command {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        self.kind.pretty(theme)
    }
}
