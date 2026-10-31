//! Every compilation phase, one after the other:
//!
//! ```text
//! source ─► lex ─► parse ─► lower ─► type ─► codegen ─► run
//!            tokens   pir      sir      tir      lir       value
//! ```
//!
//! The `Driver` keeps the state of the phases between calls (definitions of
//! the stdlib, of the previous REPL inputs, …) and reports what happens
//! through `Event`s: the result of each phase (for debugging), the errors and
//! the outputs for the user. Clients (CLI, REPL, LSP) only print events.
//!
//! Interim (v0.2.0-beta.1): the source is still lexed and parsed one command
//! at a time. After `lowering`, only `sir` is used: `typing` reads it and the
//! documentation of the definitions comes from its `DefTable`.

mod error;
mod event;

pub use event::{Event, Output};

use crate::lowering::Lowering;
use crate::typing::Typer;
use errors::Errors;
use ir::location::{SourceId, Spanned};
use ir::{pir, sir, tir};
use std::path::PathBuf;

// ============================================================================
// Driver
// ============================================================================

/// state of every phase, kept between calls to `run`
#[derive(Debug, Default)]
pub struct Driver {
    /// stop at the first error (REPL) instead of reporting every error
    /// (file, LSP)
    stop_on_error: bool,
    /// lowering of the current source; a new source starts a new one
    lowering: Lowering,
    typer: Typer,
    codegen: crate::codegen::Codegen,
    vm: crate::vm::Env,
}

impl Driver {
    /// driver with the stdlib loaded
    ///
    /// # Panics
    /// if the stdlib has an error: it is part of the compiler
    pub fn with_stdlib() -> Self {
        let mut driver = Driver::default();
        let mut errors = Vec::new();
        let source = SourceId::File(PathBuf::from("stdlib.st"));
        driver.run(source, STDLIB, 0, |event| {
            if let Event::Error(err) = event {
                errors.push(err)
            }
        });
        assert!(errors.is_empty(), "errors in the stdlib: {errors:?}");
        driver
    }

    /// stop at the first error instead of reporting every error
    pub fn with_stop_on_error(mut self, stop_on_error: bool) -> Self {
        self.stop_on_error = stop_on_error;
        self
    }

    /// typing state: environment of the definitions (hover, help, …)
    pub fn typer(&self) -> &Typer {
        &self.typer
    }

    /// definitions of the current source, with their documentation and place
    pub fn defs(&self) -> &sir::DefTable {
        self.lowering.defs()
    }

    /// run every phase on `content`, whose first byte is at `offset` in
    /// `source` (REPL inputs follow each other in one source); returns `true`
    /// if there was no error
    ///
    /// Nothing runs after an error: the following commands are still checked
    /// (unless `stop_on_error`), but no definition is computed and no `Eval`
    /// is printed.
    pub fn run(
        &mut self,
        source: SourceId,
        content: &str,
        offset: usize,
        mut on: impl FnMut(Event<'_>),
    ) -> bool {
        if self.lowering.file().source() != &source {
            self.lowering = Lowering::new(source);
        }
        let mut failed = false;
        let mut start = 0;
        while start < content.len() && !(failed && self.stop_on_error) {
            // lex: until the end of the next command
            let tokens = match crate::lexer::lex(&content[start..], offset + start) {
                Ok(tokens) => tokens,
                Err(errs) => {
                    report(&mut on, errs);
                    return false;
                }
            };
            let end = tokens.last_offset() - offset;
            on(Event::Tokens(&tokens));

            // parse
            match crate::parser::parse(tokens) {
                Ok(crate::parser::CommandOrEnd::Command(cmd)) => {
                    on(Event::Parsed(&cmd));
                    if let Err(errs) = self.command(&cmd, failed, &mut on) {
                        failed = true;
                        report(&mut on, errs);
                    }
                }
                Ok(crate::parser::CommandOrEnd::End(_)) => break,
                Err(errs) => {
                    failed = true;
                    report(&mut on, errs);
                }
            }
            start = end;
        }
        !failed
    }

    /// lower, type, generate and run one command; `failed` if an error was
    /// already found: then nothing runs
    fn command(
        &mut self,
        cmd: &pir::Command,
        failed: bool,
        on: &mut impl FnMut(Event<'_>),
    ) -> Result<(), Errors> {
        // lower
        let lowered = self.lowering.command(cmd).clone();
        on(Event::Lowered(&lowered));
        let defs = self.lowering.defs();

        // type, then generate and run
        match &lowered.kind {
            sir::CommandKind::ExpressionDefinition(def) => {
                let def = self.typer.definition(def, defs.doc(def.id))?;
                on(Event::TypedDefinition(&def));
                if !failed {
                    let global = self.codegen.definition(&def)?;
                    on(Event::Generated(&global));
                    self.vm
                        .define(&global)
                        .ok_or_else(|| error::vm(def.span()))?;
                }
                on(Event::Output(Output::Defined(def)));
            }
            sir::CommandKind::TypeDefinition(def) => {
                self.typer.type_definition(def, defs.doc(def.id))?;
            }
            sir::CommandKind::Eval(expr) => {
                let expr = self.typer.expression(expr)?;
                on(Event::TypedExpression(&expr));
                if !failed {
                    let code = self.codegen.expression(&expr)?;
                    on(Event::GeneratedExpression(&code));
                    let value = self.vm.eval(&code).ok_or_else(|| error::vm(expr.span()))?;
                    on(Event::Output(Output::Value(value, expr.span())));
                }
            }
            sir::CommandKind::TypeOf(expr) => {
                let expr = self.typer.expression(expr)?;
                on(Event::TypedExpression(&expr));
                on(Event::Output(Output::Type(tir::Typed::ty(&expr).clone())));
            }
            sir::CommandKind::Help(var) => {
                let help = self.typer.help(var)?;
                on(Event::Output(Output::Help(help)));
            }
            sir::CommandKind::Set(var) => on(Event::Output(Output::Option {
                name: var.clone(),
                on: true,
            })),
            sir::CommandKind::UnSet(var) => on(Event::Output(Output::Option {
                name: var.clone(),
                on: false,
            })),
        }
        Ok(())
    }
}

/// report every error of a phase
fn report(on: &mut impl FnMut(Event<'_>), errs: Errors) {
    for err in errs {
        on(Event::Error(err));
    }
}

/// source of the stdlib, loaded by `Driver::with_stdlib`
const STDLIB: &str = include_str!("../../../assets/stdlib.st");

#[cfg(test)]
mod tests {
    use super::*;
    use ir::tir::Typed as _;

    /// what a run reported, without the borrowed parts
    #[derive(Debug, Default)]
    struct Report {
        /// phases, in order: `tokens`, `pir`, `sir`, `tir`, `lir`
        phases: Vec<&'static str>,
        errors: Vec<i32>,
        values: Vec<crate::vm::Value>,
        types: Vec<String>,
        helps: usize,
        defined: Vec<String>,
        options: Vec<(String, bool)>,
        ok: bool,
    }

    fn run(driver: &mut Driver, source: SourceId, content: &str, offset: usize) -> Report {
        let mut report = Report::default();
        report.ok = driver.run(source, content, offset, |event| match event {
            Event::Tokens(_) => report.phases.push("tokens"),
            Event::Parsed(_) => report.phases.push("pir"),
            Event::Lowered(_) => report.phases.push("sir"),
            Event::TypedDefinition(_) | Event::TypedExpression(_) => report.phases.push("tir"),
            Event::Generated(_) | Event::GeneratedExpression(_) => report.phases.push("lir"),
            Event::Error(err) => report.errors.push(err.code()),
            Event::Output(Output::Value(value, _)) => report.values.push(value),
            Event::Output(Output::Type(ty)) => report.types.push(format!("{ty:?}")),
            Event::Output(Output::Help(_)) => report.helps += 1,
            Event::Output(Output::Defined(def)) => {
                report.defined.push(format!("{:?}", def.ty()));
            }
            Event::Output(Output::Option { name, on }) => {
                report.options.push((name.value, on));
            }
        });
        report
    }

    fn run_file(content: &str) -> Report {
        run(&mut Driver::with_stdlib(), SourceId::Repl, content, 0)
    }

    fn nat(n: u32) -> crate::vm::Value {
        crate::vm::Value::Nat(n.into())
    }

    #[test]
    fn every_phase_in_order() {
        let report = run_file("Def a := 3.\nEval a.");
        assert!(report.ok);
        assert_eq!(
            report.phases,
            [
                "tokens", "pir", "sir", "tir", "lir", "tokens", "pir", "sir", "tir", "lir"
            ]
        );
        assert_eq!(report.values, [nat(3)]);
        assert_eq!(report.defined.len(), 1);
    }

    #[test]
    fn uses_the_stdlib() {
        let report = run_file("Eval true.\nTypeOf true.\nHelp true.");
        assert!(report.ok);
        assert_eq!(report.values, [crate::vm::Value::Bool(true)]);
        assert_eq!(report.types.len(), 1);
        assert_eq!(report.helps, 1);
    }

    #[test]
    fn redefinition() {
        let report = run_file("Def a := 3.\nDef b := a.\nDef a := 'x'.\nEval b.\nEval a.");
        assert!(report.ok);
        assert_eq!(report.values, [nat(3), crate::vm::Value::Char('x')]);
    }

    #[test]
    fn options_are_left_to_the_client() {
        let report = run_file("Set DebugParser.\nUnset Foo.");
        assert!(report.ok);
        assert_eq!(
            report.options,
            [
                ("DebugParser".to_string(), true),
                ("Foo".to_string(), false)
            ]
        );
    }

    #[test]
    fn nothing_runs_after_an_error() {
        // `x` is unknown; the definition after it is typed but not run, and
        // the `Eval` is not printed
        let report = run_file("Eval x.\nDef a := 1.\nEval a.\nTypeOf a.");
        assert!(!report.ok);
        assert_eq!(report.errors, [301]);
        assert!(report.values.is_empty());
        assert_eq!(report.defined.len(), 1);
        assert_eq!(report.types.len(), 1);
        assert!(!report.phases.contains(&"lir"));
    }

    #[test]
    fn every_error_is_reported() {
        let report = run_file("Eval x.\nEval y.");
        assert_eq!(report.errors, [301, 301]);
    }

    #[test]
    fn stop_on_error() {
        let mut driver = Driver::with_stdlib().with_stop_on_error(true);
        let report = run(&mut driver, SourceId::Repl, "Eval x.\nEval y.", 0);
        assert_eq!(report.errors, [301]);
    }

    #[test]
    fn lexing_error_stops() {
        let report = run_file("Eval 'ab'.\nEval 1.");
        assert!(!report.ok);
        assert_eq!(report.errors.len(), 1);
        assert!(report.values.is_empty());
    }

    #[test]
    fn state_kept_between_runs() {
        // REPL: each input is one run, at the end of the previous ones
        let mut driver = Driver::with_stdlib();
        let first = "Def a := 2.\n";
        assert!(run(&mut driver, SourceId::Repl, first, 0).ok);
        let report = run(&mut driver, SourceId::Repl, "Eval a.", first.len());
        assert_eq!(report.values, [nat(2)]);
        assert_eq!(driver.defs().all("a").len(), 1);
    }

    #[test]
    fn documentation_from_the_def_table() {
        let mut driver = Driver::with_stdlib();
        assert!(run(&mut driver, SourceId::Repl, "(** two *)\nDef a := 2.", 0).ok);
        let id = driver.defs().last("a").unwrap();
        assert_eq!(
            driver.defs().doc(id).map(|doc| doc.lines().to_vec()),
            Some(vec!["two".to_string()])
        );
        let info = driver
            .typer()
            .env()
            .iter()
            .find(|info| info.id.name() == "a");
        assert!(info.and_then(|info| info.doc.as_ref()).is_some());
    }

    #[test]
    fn new_source_new_def_table() {
        let mut driver = Driver::with_stdlib();
        let stdlib_defs = driver.defs().len();
        assert!(stdlib_defs > 0);
        assert!(run(&mut driver, SourceId::Repl, "Def a := 2.", 0).ok);
        assert_eq!(driver.defs().len(), 1);
        assert_eq!(driver.defs().source(), &SourceId::Repl);
    }
}
