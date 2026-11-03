//! The state of a project: the passes that keep one (stdlib, files, REPL
//! inputs), and every pass on a text from that state.

use crate::Checked;
use crate::codegen::Codegen;
use crate::lexer::Lexer;
use crate::lowering::Lowerer;
use crate::parser::Parser;
use crate::typing::Typer;
use crate::vm::{Value, Vm};
use errors::Errors;
use ir::location::SourceId;
use ir::tir;
use std::path::PathBuf;

/// source of the stdlib
pub const SOURCE: &str = include_str!("../../assets/stdlib.st");

/// a text that went through every pass
pub struct Compiled {
    pub tir: Checked<tir::File>,
    /// value of each `Eval`, in order
    pub values: Checked<Vec<Value>>,
}

/// state of a project, with the stdlib loaded; a clone is a snapshot, to go
/// back to after a text with errors
#[derive(Clone)]
pub struct Stdlib {
    pub lowerer: Lowerer,
    pub typer: Typer,
    pub codegen: Codegen,
    pub vm: Vm,
}

impl Stdlib {
    /// run the stdlib through every pass
    ///
    /// # Panics
    /// if the stdlib has an error: it is part of the compiler
    pub fn load() -> Self {
        let mut stdlib = Stdlib {
            lowerer: Lowerer::new(),
            typer: Typer::new(),
            codegen: Codegen::new(),
            vm: Vm::new(),
        };
        let source = SourceId::File(PathBuf::from("stdlib.st"));
        stdlib
            .run(source, SOURCE, 0)
            .expect("the stdlib has no error");
        stdlib
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
        let tokens = Lexer::builder().offset(offset).build().run(text)?;
        let pir = Parser::builder().source(source).build().run(&tokens)?;
        let sir = self.lowerer.run(&pir)?;
        let tir = self.typer.run(&sir)?;
        let lir = self.codegen.run(&tir)?;
        let values = self.vm.run(&lir)?;
        Ok(Compiled { tir, values })
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
