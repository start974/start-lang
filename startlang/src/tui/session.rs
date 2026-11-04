//! A session of `startlang tui`, without terminal: each input goes through
//! every pass after the previous ones, and gives the parts of its frame: its
//! commands, each with its summary, or its errors.

use super::output::{Output, Part, styled};
use crate::printer::Printer;
use errors::Errors;
use ir::location::{SourceId, Spanned};
use ir::tir;
use pass::stdlib::{Compiled, Stdlib};
use pass::vm::Value;
use pp::StreamColored;
use pp::pretty::*;
use ratatui::text::Text;

// ============================================================================
// Session
// ============================================================================

/// the inputs run so far, and what they defined
pub struct Session {
    /// definitions of the stdlib and of the previous inputs
    state: Stdlib,
    /// inputs run so far: the next one is `input n+1`
    inputs: usize,
    /// colors of the commands, summaries and errors (none when the output is
    /// not a terminal)
    theme: Theme,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            state: Stdlib::load(),
            inputs: 0,
            theme: Theme::colored(),
        }
    }
}

impl Session {
    /// run `input` after the previous inputs, as a source of its own: each
    /// command formatted, with its summary under it; or the input as typed,
    /// with its errors (then nothing of it is kept)
    pub(super) fn run(&mut self, input: &str) -> Output {
        self.inputs += 1;
        let source = SourceId::Url(format!("input {}", self.inputs));
        let parts = match self.state.run(source.clone(), input, 0) {
            Ok(compiled) => self.commands(compiled),
            Err(errors) => self.failure(&source, input, errors),
        };
        Output { source, parts }
    }

    /// each command formatted (the line numbers going on from one to the
    /// next), then the summary of its typed command, if it has one: the
    /// typed commands come in order with the same span, without the type
    /// definitions
    fn commands(&self, compiled: Compiled) -> Vec<Part> {
        let Compiled { pir, tir, values } = compiled;
        let mut values = values.iter().cloned();
        let mut typed = tir.commands.iter().peekable();
        pir.into_inner()
            .commands
            .into_iter()
            .scan(0, |first, command| {
                let code = Part::Code {
                    first: *first,
                    text: styled(&command.make_string(&self.theme)),
                };
                *first += code.height() as usize;
                let summary = typed
                    .next_if(|typed| typed.span == command.span())
                    .map(|typed| self.summary(typed, &mut values));
                Some(std::iter::once(code).chain(summary))
            })
            .flatten()
            .collect()
    }

    /// the summary of a typed command: `a : N`, `3 : N` (an `Eval` takes the
    /// next value of `values`), `N`, or the help
    fn summary(&self, command: &tir::Command, values: &mut impl Iterator<Item = Value>) -> Part {
        let theme = &self.theme;
        let text = match &command.kind {
            tir::CommandKind::ExpressionDefinition(def) => {
                let tir::Pattern::Variable(var) = &def.pattern;
                render(
                    typed(theme.def_var(&var.value.name), &def.body.ty, theme),
                    theme,
                )
            }
            tir::CommandKind::Eval(expr) => {
                let value = values.next().expect("the vm computes a value per Eval");
                render(typed(value.pretty(theme), &expr.ty, theme), theme)
            }
            tir::CommandKind::TypeOf(expr) => expr.ty.make_string(theme),
            tir::CommandKind::Help(help) => help.make_string(theme),
        };
        Part::Summary(styled(&text))
    }

    /// the input as typed (it may not parse), then its errors in source
    /// order, with their place in it
    fn failure(&self, source: &SourceId, input: &str, errors: Errors) -> Vec<Part> {
        let mut printer = Printer::new(source.clone()).with_theme(self.theme.clone());
        printer.set_content(input);
        let report: String = errors
            .sorted()
            .into_iter()
            .map(|err| printer.render_error(&err))
            .collect();
        vec![
            Part::Code {
                first: 0,
                text: Text::raw(input.to_string()),
            },
            Part::Error(styled(&report)),
        ]
    }
}

/// `doc : ty`
fn typed<'a>(doc: Doc<'a>, ty: &'a tir::Type, theme: &Theme) -> Doc<'a> {
    doc.append(Doc::space())
        .append(theme.operator(&":"))
        .append(Doc::space())
        .append(ty.pretty(theme))
}

/// the text of `doc`, in the colors of `theme`
fn render(doc: Doc<'_>, theme: &Theme) -> String {
    let mut text = String::new();
    doc.render_raw(theme.width, &mut StreamColored::new(&mut text))
        .expect("writing to a String cannot fail");
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> Session {
        Session::default()
    }

    /// a part as plain text: `Code` / `Summary` / `Error`, and its text
    fn plain(part: &Part) -> (&'static str, String) {
        let text = |text: &Text| {
            text.lines
                .iter()
                .map(|line| {
                    line.spans
                        .iter()
                        .map(|span| span.content.as_ref())
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        match part {
            Part::Code { text: code, .. } => ("Code", text(code)),
            Part::Summary(summary) => ("Summary", text(summary)),
            Part::Error(error) => ("Error", text(error)),
        }
    }

    /// the parts of `input`, run in `session`, as plain text
    fn parts(session: &mut Session, input: &str) -> Vec<(&'static str, String)> {
        session.run(input).parts.iter().map(plain).collect()
    }

    fn part(kind: &'static str, text: &str) -> (&'static str, String) {
        (kind, text.to_string())
    }

    #[test]
    fn each_command_formatted_with_its_summary() {
        let mut session = session();
        assert_eq!(
            parts(
                &mut session,
                "Type   N:=Nat.\nDef a : N := 3.\n$ a. TypeOf a."
            ),
            [
                // a type definition has no summary
                part("Code", "Type N := Nat."),
                part("Code", "Def a : N := 3."),
                part("Summary", "a : N"),
                // the command as written (`$`, not `Eval`)
                part("Code", "$ a."),
                part("Summary", "3 : N"),
                part("Code", "TypeOf a."),
                part("Summary", "N"),
            ]
        );
        assert_eq!(
            parts(&mut session, "(** the answer *)\nDef b := 42.\nHelp b."),
            [
                part("Code", "(** the answer *)\nDef b := 42."),
                part("Summary", "b : ℕ"),
                part("Code", "Help b."),
                part("Summary", "b : ℕ\n  the answer"),
            ]
        );
    }

    #[test]
    fn line_numbers_going_on() {
        let mut session = session();
        let output = session.run("Def a := 1.\nEval a.");
        let firsts: Vec<_> = output
            .parts
            .iter()
            .filter_map(|part| match part {
                Part::Code { first, .. } => Some(*first),
                _ => None,
            })
            .collect();
        assert_eq!(firsts, [0, 1]);
    }

    #[test]
    fn each_input_its_own_source() {
        let mut session = session();
        assert_eq!(
            session.run("Def a := 1.").source,
            SourceId::Url("input 1".to_string())
        );
        // a definition of an input is used by the next ones
        let second = session.run("Eval a.");
        assert_eq!(second.source, SourceId::Url("input 2".to_string()));
        assert!(matches!(second.parts.as_slice(), [_, Part::Summary(_)]));
    }

    #[test]
    fn errors_with_their_place_in_the_input() {
        let mut session = session();
        session.run("Eval 1.");
        let parts = parts(&mut session, "Def a := 3.   Eval not_exits.");
        // the input as typed, then its errors
        assert_eq!(parts[0], part("Code", "Def a := 3.   Eval not_exits."));
        let (kind, report) = &parts[1];
        assert_eq!(*kind, "Error");
        assert!(
            report.contains("[301] Error: Variable not found."),
            "{report}"
        );
        // in the second input, its first line
        assert!(report.contains("input 2:1:20"), "{report}");
        assert_eq!(parts.len(), 2);
    }

    #[test]
    fn input_with_an_error_is_forgotten() {
        let mut session = session();
        session.run("Def a := 3. Eval not_exits.");
        let parts = parts(&mut session, "Eval a.");
        assert!(
            parts[1].1.contains("Expression variable a not found."),
            "{parts:?}"
        );
    }
}
