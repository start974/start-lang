//! `startlang run <file>`: every pass on the file.

use crate::compile::compile;
use crate::error;
use crate::printer::Printer;
use driver::stdlib::Stdlib;
use ir::location::SourceId;
use std::path::Path;

/// run the file at `path`; returns the exit code: the code of the only
/// error, `1` for several, `0` for none
pub fn run(path: &Path) -> i32 {
    let source = SourceId::File(path.to_path_buf());
    let mut printer = Printer::new(source.clone());
    let Ok(content) = std::fs::read_to_string(path) else {
        printer.fail(&error::read_file(path));
        return printer.code();
    };
    printer.set_content(content.clone());

    match compile(&mut Stdlib::load(), source, &content, 0) {
        Ok(compiled) => compiled
            .outputs()
            .for_each(|output| printer.output(&output)),
        Err(errs) => printer.errors(errs),
    }
    printer.code()
}
