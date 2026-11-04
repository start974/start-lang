//! A session of `startlang tui`, without terminal: each input goes through
//! every pass after the previous ones, and gives its typed commands, each
//! with its summary, or its errors.

use crate::printer::Printer;
use errors::Errors;
use ir::location::SourceId;
use ir::tir;
use pass::stdlib::{Compiled, Stdlib};
use pass::vm::Value;
use pp::pretty::*;

// ============================================================================
// Summary
// ============================================================================

/// what a typed command gives, one variant per kind of command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Summary {
    /// `Def a : N := 3.` → `a : N`
    Defined { name: String, ty: tir::Type },
    /// `Eval a.` → `3 : N`
    Value { value: Value, ty: tir::Type },
    /// `TypeOf a.` → `N`
    Type(tir::Type),
    /// `Help a.` → `a : N` and its documentation
    Help(tir::Help),
}

impl Summary {
    /// the summary of `command`; `value`: the value computed for an `Eval`
    ///
    /// # Panics
    /// if `command` is an `Eval` without value: the vm computes one per `Eval`
    pub fn new(command: &tir::Command, value: Option<Value>) -> Self {
        match &command.kind {
            tir::CommandKind::ExpressionDefinition(def) => {
                let tir::Pattern::Variable(var) = &def.pattern;
                Summary::Defined {
                    name: var.value.name.clone(),
                    ty: def.body.ty.clone(),
                }
            }
            tir::CommandKind::Eval(expr) => Summary::Value {
                value: value.expect("the vm computes a value per Eval"),
                ty: expr.ty.clone(),
            },
            tir::CommandKind::TypeOf(expr) => Summary::Type(expr.ty.clone()),
            tir::CommandKind::Help(help) => Summary::Help(help.clone()),
        }
    }
}

impl Pretty for Summary {
    fn pretty(&self, theme: &Theme) -> Doc<'_> {
        let (doc, ty) = match self {
            Summary::Defined { name, ty } => (theme.def_var(name), ty),
            Summary::Value { value, ty } => (value.pretty(theme), ty),
            Summary::Type(ty) => return ty.pretty(theme),
            Summary::Help(help) => return help.pretty(theme),
        };
        doc.append(Doc::space())
            .append(theme.operator(&":"))
            .append(Doc::space())
            .append(ty.pretty(theme))
    }
}

// ============================================================================
// Session
// ============================================================================

/// the inputs run so far, and what they defined
pub struct Session {
    /// definitions of the stdlib and of the previous inputs
    state: Stdlib,
    /// colors, and errors with their place in `content`
    printer: Printer,
    /// every input so far, one after the other: the source of the errors
    content: String,
}

impl Session {
    pub fn new(theme: Theme) -> Self {
        Self {
            state: Stdlib::load(),
            printer: Printer::new(SourceId::Repl).with_theme(theme),
            content: String::new(),
        }
    }

    /// run `input` after the previous ones: each of its typed commands with
    /// its summary (a type definition leaves no typed command), or its errors
    /// (then nothing of `input` is kept)
    pub fn run(&mut self, input: &str) -> Result<Vec<(tir::Command, Summary)>, Errors> {
        let offset = self.content.len();
        self.content.push_str(input);
        self.content.push('\n');
        self.printer.set_content(self.content.clone());
        self.state
            .run(SourceId::Repl, input, offset)
            .map(|compiled| summaries(&compiled))
    }

    /// the text of the errors of an input, in source order, with their place
    /// in the inputs
    pub fn report(&self, errs: Errors) -> String {
        errs.sorted()
            .into_iter()
            .map(|err| self.printer.render_error(&err))
            .collect()
    }
}

/// each typed command with its summary; the value of an `Eval` is the next
/// value computed
fn summaries(compiled: &Compiled) -> Vec<(tir::Command, Summary)> {
    let mut values = compiled.values.iter().cloned();
    compiled
        .tir
        .commands
        .iter()
        .map(|command| {
            let value = match command.kind {
                tir::CommandKind::Eval(_) => values.next(),
                _ => None,
            };
            (command.clone(), Summary::new(command, value))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::tir::ty::BuiltinType;

    fn session() -> Session {
        Session::new(Theme::default())
    }

    /// the summaries of `input`, run in `session`
    fn summaries(session: &mut Session, input: &str) -> Vec<Summary> {
        let entries = session.run(input).expect("no error");
        entries.into_iter().map(|(_, summary)| summary).collect()
    }

    const NAT: tir::Type = tir::Type::Builtin(BuiltinType::Nat);

    #[test]
    fn a_summary_for_each_command() {
        let mut session = session();
        // a type definition leaves no typed command
        assert!(summaries(&mut session, "Type N1 := Nat.").is_empty());
        assert!(matches!(
            summaries(&mut session, "Def a : N1 := 3.").as_slice(),
            [Summary::Defined { name, ty }] if name == "a" && ty.builtin() == BuiltinType::Nat
        ));
        assert!(matches!(
            summaries(&mut session, "Eval a.").as_slice(),
            [Summary::Value { value: pass::vm::Value::Nat(n), .. }] if *n == 3u8.into()
        ));
        assert!(matches!(
            summaries(&mut session, "TypeOf a.").as_slice(),
            [Summary::Type(ty)] if ty.builtin() == BuiltinType::Nat
        ));
        assert!(matches!(
            summaries(&mut session, "Help a.").as_slice(),
            [Summary::Help(help)] if help.def.id.name == "a"
        ));
    }

    #[test]
    fn commands_and_summaries_in_order() {
        let mut session = session();
        let entries = session
            .run("Def a := 3.\n$ a. TypeOf a.")
            .expect("no error");
        let kinds: Vec<_> = entries.iter().map(|(command, _)| &command.kind).collect();
        assert!(matches!(
            kinds.as_slice(),
            [
                tir::CommandKind::ExpressionDefinition(_),
                tir::CommandKind::Eval(_),
                tir::CommandKind::TypeOf(_)
            ]
        ));
        assert_eq!(entries[2].1, Summary::Type(NAT));
    }

    #[test]
    fn summary_text() {
        let mut session = session();
        let text = |input: &str, session: &mut Session| -> Vec<String> {
            summaries(session, input)
                .iter()
                .map(|summary| summary.make_string(&Theme::default()))
                .collect()
        };
        assert_eq!(
            text("Type N1 := Nat.\nDef a : N1 := 3.", &mut session),
            ["a : N1"]
        );
        assert_eq!(text("Eval a.\nTypeOf a.", &mut session), ["3 : N1", "N1"]);
        assert_eq!(
            text("(** the answer *)\nDef b := 42.\nHelp b.", &mut session),
            ["b : ℕ", "b : ℕ\n  the answer"]
        );
    }

    #[test]
    fn errors_with_their_place_in_the_inputs() {
        let mut session = session();
        assert!(session.run("Eval 1.").is_ok());
        let errs = session.run("Def a := 3. Eval not_exits.").unwrap_err();
        assert_eq!(errs.code(), 301);
        let text = session.report(errs);
        assert!(text.contains("[301] Error: Variable not found."), "{text}");
        // the second input, first line
        assert!(text.contains("REPL:2:18"), "{text}");
        assert!(text.contains("Def a := 3. Eval not_exits."), "{text}");
    }

    #[test]
    fn input_with_an_error_is_forgotten() {
        let mut session = session();
        assert!(session.run("Def a := 3. Eval not_exits.").is_err());
        let errs = session.run("Eval a.").unwrap_err();
        let text = session.report(errs);
        assert!(text.contains("Expression variable a not found."), "{text}");
    }
}
