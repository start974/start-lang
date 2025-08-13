use super::error::ErrorFileRead;
use crate::interpreter;
use crate::interpreter::flag::DebugFlag;
use crate::interpreter::flag::Flag;
use crate::interpreter::Interpreter as _;
use crate::typer::Typer;
use crate::utils::error::{ErrorPrint as _, ErrorReport};
use crate::utils::location::SourceId;
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use crate::vm;
use ariadne::Source;
use std::fs::read_to_string;
use std::path::Path;
use std::path::PathBuf;

pub struct Interpreter {
    source_id: SourceId,
    content: String,
    err_code: i32,
    pub typer: Typer,
    pub vm: vm::Env,
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
        match read_to_string(path) {
            Ok(content) => {
                interpreter.content = content;
            }
            Err(_) => interpreter.fail(ErrorFileRead::new(path.to_path_buf())),
        }
        interpreter
    }

    /// stdlib environement
    pub fn stdlib() -> Self {
        Interpreter {
            source_id: SourceId::File(PathBuf::from("stdlib.st")),
            content: include_str!("../../../assets/stdlib.st").to_string(),
            err_code: 0,
            typer: Typer::default(),
            vm: vm::Env::default(),
            debug_lexer: false,
            debug_parser: false,
            debug_typer: false,
            theme: Theme::default_theme(),
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

    fn get_error_code(&self) -> i32 {
        self.err_code
    }

    fn continue_parsing(&self) -> bool {
        true
    }

    fn get_offset_source(&self, offset: usize) -> usize {
        offset
    }

    fn mut_typer(&mut self) -> &mut Typer {
        &mut self.typer
    }

    fn mut_vm(&mut self) -> &mut vm::Env {
        &mut self.vm
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

    fn print_summay(&self, _: &crate::typer::ast::ExpressionDefinition) {}

    fn eprint<E>(&mut self, error: &E)
    where
        E: ErrorReport,
    {
        let mut cache = (self.source_id.clone(), Source::from(&self.content));
        error.eprint(&self.theme, &mut cache).unwrap();
    }

    fn print<Doc>(&mut self, doc: &Doc)
    where
        Doc: Pretty,
    {
        println!("{}", doc.make_string(&self.theme));
    }
}
