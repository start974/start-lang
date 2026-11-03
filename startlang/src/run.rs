//! `startlang run <file>`: every pass on the file.

use crate::compile::outputs;
use crate::error;
use crate::printer::Printer;
use ir::location::SourceId;
use pass::stdlib::Stdlib;
use std::path::Path;

/// a phase whose IR `--debug` prints
#[derive(clap::ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Debug {
    Lexer,
    Parser,
    Lowering,
    Typer,
    Codegen,
}

/// run the file at `path`, printing first the IRs of the phases in `debug`;
/// returns the exit code: the code of the only error, `1` for several, `0`
/// for none
pub fn run(path: &Path, debug: &[Debug]) -> i32 {
    let source = SourceId::File(path.to_path_buf());
    let mut printer = Printer::new(source.clone());
    let Ok(content) = std::fs::read_to_string(path) else {
        printer.fail(&error::read_file(path));
        return printer.code();
    };
    printer.set_content(content.clone());

    let debug = pass::stdlib::Debug {
        lexer: debug.contains(&Debug::Lexer),
        parser: debug.contains(&Debug::Parser),
        lowering: debug.contains(&Debug::Lowering),
        typer: debug.contains(&Debug::Typer),
        codegen: debug.contains(&Debug::Codegen),
    };
    match Stdlib::load().debug(debug).run(source, &content, 0) {
        Ok(compiled) => outputs(&compiled).for_each(|output| printer.output(&output)),
        Err(errs) => printer.errors(errs),
    }
    printer.code()
}
