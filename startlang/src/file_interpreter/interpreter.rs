use crate::error;
use crate::interpreter::{
    self, Interpreter as _, Runtime,
    flag::{DebugFlag, Flag},
};
use ariadne::Source;
use driver::lowering::Lowering;
use driver::typing::Typer;
use errors::Error;
use ir::location::SourceId;
use ir::tir;
use pp::{pretty::Pretty, theme::Theme};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub struct Interpreter {
    source_id: SourceId,
    content: String,
    err_code: i32,
    lowering: Lowering,
    pub typer: Typer,
    pub runtime: Runtime,
    debug_lexer: bool,
    debug_parser: bool,
    debug_typer: bool,
    theme: Theme,
}

impl Interpreter {
    pub fn new(path: &Path) -> Self {
        let mut interpreter = Interpreter::stdlib();
        interpreter.run();
        interpreter.source_id = SourceId::File(path.to_path_buf());
        interpreter.lowering = Lowering::new(interpreter.source_id.clone());
        match read_to_string(path) {
            Ok(content) => {
                interpreter.content = content;
            }
            Err(_) => interpreter.fail(error::read_file(path)),
        }
        interpreter
    }

    /// stdlib environment
    pub fn stdlib() -> Self {
        Interpreter {
            source_id: SourceId::File(PathBuf::from("stdlib.st")),
            content: include_str!("../../../assets/stdlib.st").to_string(),
            err_code: 0,
            lowering: Lowering::new(SourceId::File(PathBuf::from("stdlib.st"))),
            typer: Typer::default(),
            runtime: Runtime::default(),
            debug_lexer: false,
            debug_parser: false,
            debug_typer: false,
            theme: Theme::colored(),
        }
    }
}

impl interpreter::Interpreter for Interpreter {
    fn content(&self) -> &str {
        &self.content
    }

    fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    fn set_error_code(&mut self, code: i32) {
        self.err_code = code;
    }

    fn error_code(&self) -> i32 {
        self.err_code
    }

    fn continue_parsing(&self) -> bool {
        true
    }

    fn offset_source(&self, offset: usize) -> usize {
        offset
    }

    fn mut_lowering(&mut self) -> &mut Lowering {
        &mut self.lowering
    }

    fn mut_typer(&mut self) -> &mut Typer {
        &mut self.typer
    }

    fn mut_runtime(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    fn set_flag(&mut self, b: bool, flag: Flag) {
        match flag {
            Flag::Debug(DebugFlag::Lexer) => self.debug_lexer = b,
            Flag::Debug(DebugFlag::Parser) => self.debug_parser = b,
            Flag::Debug(DebugFlag::Typer) => self.debug_typer = b,
        }
    }

    fn is_active_debug(&self, debug: DebugFlag) -> bool {
        match debug {
            DebugFlag::Lexer => self.debug_lexer,
            DebugFlag::Parser => self.debug_parser,
            DebugFlag::Typer => self.debug_typer,
        }
    }

    fn print_summary(&self, _: &tir::ExpressionDefinition) {}

    fn eprint(&mut self, error: &Error) {
        let mut cache = (self.source_id.clone(), Source::from(&self.content));
        error.eprint(&self.source_id, &self.theme, &mut cache)
    }

    fn print<Doc>(&mut self, doc: &Doc)
    where
        Doc: Pretty,
    {
        println!("{}", doc.make_string(&self.theme));
    }
}
