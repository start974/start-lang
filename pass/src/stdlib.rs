//! The state of a project: the passes that keep one (stdlib, files, REPL
//! inputs), and every pass on a text from that state.

use crate::codegen::Codegen;
use crate::lexer::Lexer;
use crate::lowering::Lowerer;
use crate::parser::Parser;
use crate::typing::Typer;
use crate::vm::{Value, Vm};
use crate::{Checked, CheckedResult, Partial};
use errors::{Errors, Message};
use ir::location::SourceId;
use ir::{pir, tir};
use std::path::PathBuf;

/// source of the stdlib
pub const SOURCE: &str = include_str!("../../assets/stdlib.st");

/// a text that went through every pass
pub struct Compiled {
    /// the commands as written (to format them), in source order
    pub pir: Checked<pir::File>,
    pub tir: Checked<tir::File>,
    /// value of each `Eval`, in order
    pub values: Checked<Vec<Value>>,
}

/// the passes that print the IR they built on stdout, when a text goes
/// through them (`startlang run --debug`)
#[derive(Debug, Default, Clone, Copy)]
pub struct Debug {
    pub lexer: bool,
    pub parser: bool,
    pub lowering: bool,
    pub typer: bool,
    pub codegen: bool,
}

/// state of a project, with the stdlib loaded; a clone is a snapshot, to go
/// back to after a text with errors
#[derive(Clone)]
pub struct Stdlib {
    pub lowerer: Lowerer,
    pub typer: Typer,
    pub codegen: Codegen,
    pub vm: Vm,
    /// flags of the lexer and the parser, built for each text
    debug: Debug,
}

impl Stdlib {
    /// run the stdlib through every pass
    ///
    /// # Panics
    /// if the stdlib has an error: it is part of the compiler
    pub fn load() -> Self {
        let mut stdlib = Stdlib {
            lowerer: Lowerer::builder().build(),
            typer: Typer::builder().build(),
            codegen: Codegen::builder().build(),
            vm: Vm::new(),
            debug: Debug::default(),
        };
        let source = SourceId::File(PathBuf::from("stdlib.st"));
        stdlib
            .run(source, SOURCE, 0)
            .expect("the stdlib has no error");
        stdlib
    }

    /// the passes in `debug` print their IR for the next texts (not for the
    /// stdlib)
    pub fn debug(mut self, debug: Debug) -> Self {
        self.lowerer.debug = debug.lowering;
        self.typer.debug = debug.typer;
        self.codegen.debug = debug.codegen;
        self.debug = debug;
        self
    }

    /// every pass on `text`, whose first byte is at `offset` in `source`;
    /// the next pass only runs if there is no error, and on error the state
    /// is left as it was before `text`
    pub fn run(&mut self, source: SourceId, text: &str, offset: usize) -> Result<Compiled, Errors> {
        let before = self.clone();
        let compiled = self.passes(source, text, offset);
        if compiled.is_err() {
            *self = before;
        }
        compiled
    }

    fn passes(&mut self, source: SourceId, text: &str, offset: usize) -> Result<Compiled, Errors> {
        let tokens = Lexer::builder()
            .offset(offset)
            .debug(self.debug.lexer)
            .build()
            .run(text)?;
        let pir = Parser::builder()
            .source(source)
            .debug(self.debug.parser)
            .build()
            .run(&tokens)?;
        let sir = self.lowerer.run(&pir)?;
        let tir = self.typer.run(&sir)?;
        let lir = self.codegen.run(&tir)?;
        self.vm.run(&lir)?;
        let values = self.evaluate(&tir)?;
        Ok(Compiled { pir, tir, values })
    }

    /// the value of each `Eval` of `tir`, in order: generated, then evaluated
    /// by the vm from the globals
    fn evaluate(&self, tir: &tir::File) -> CheckedResult<Vec<Value>> {
        let values: Partial<Vec<_>> = tir
            .commands
            .iter()
            .filter_map(|cmd| match &cmd.kind {
                tir::CommandKind::Eval(expr) => Some(expr),
                _ => None,
            })
            .map(|expr| {
                let value = self
                    .codegen
                    .expression(expr)
                    .map_err(|errs| {
                        errs.into_iter()
                            .map(|err| err.with_recovery(Message::text("the command is left out")))
                            .collect()
                    })
                    .and_then(|expr| self.vm.evaluate(&expr));
                match value {
                    Ok(value) => Partial::ok(Some(value)),
                    Err(errs) => Partial::ok(None).with_errors(errs),
                }
            })
            .collect();
        values
            .map(|values| values.into_iter().flatten().collect())
            .check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stdlib_loads() {
        let stdlib = Stdlib::load();
        let names: Vec<_> = stdlib
            .typer
            .defs()
            .map(|info| info.id.name.as_str())
            .collect();
        assert!(names.contains(&"true") && names.contains(&"ℕ"));
    }

    #[test]
    fn error_leaves_state_as_before() {
        let mut stdlib = Stdlib::load();
        assert!(stdlib.run(SourceId::Repl, "Def a := 1.", 0).is_ok());
        assert!(
            stdlib
                .run(SourceId::Repl, "Def b := 2.\nEval c.", 0)
                .is_err()
        );
        assert!(stdlib.run(SourceId::Repl, "Eval b.", 0).is_err());
        let compiled = stdlib.run(SourceId::Repl, "Eval a.", 0);
        assert_eq!(compiled.map(|compiled| compiled.values.len()).ok(), Some(1));
    }
}
