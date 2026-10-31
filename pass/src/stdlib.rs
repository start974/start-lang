//! The stdlib through every pass: the state the tools start from.

use crate::codegen::Codegen;
use crate::lexer::Lexer;
use crate::lowering::Lowering;
use crate::parser::Parser;
use crate::typing::Typer;
use crate::vm::Vm;
use ir::location::SourceId;
use std::path::PathBuf;

/// source of the stdlib
pub const SOURCE: &str = include_str!("../../assets/stdlib.st");

/// state of the passes that keep one, with the stdlib loaded
#[derive(Debug)]
pub struct Stdlib {
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
        let source = SourceId::File(PathBuf::from("stdlib.st"));
        let mut typer = Typer::builder().build();
        let mut codegen = Codegen::builder().build();
        let mut vm = Vm::builder().build();

        let tokens = Lexer::builder().build().run(SOURCE).expect("stdlib lexing");
        let pir = Parser::builder()
            .build()
            .run(tokens)
            .expect("stdlib parsing");
        let sir = Lowering::builder().source(source).build().run(&pir);
        let tir = typer.run(&sir).expect("stdlib typing");
        let lir = codegen.run(&tir).expect("stdlib codegen");
        vm.run(&lir).expect("stdlib run");

        Self { typer, codegen, vm }
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
            .env()
            .iter()
            .map(|info| info.id.name())
            .collect();
        assert!(names.contains(&"true") && names.contains(&"ℕ"));
    }
}
