use super::summary::SummaryDefinition;
use crate::file_interpreter;
use crate::interpreter;
use crate::interpreter::Interpreter as _;
use crate::interpreter::Runtime;
use crate::interpreter::flag::DebugFlag;
use crate::interpreter::flag::Flag;
use ariadne::Source;
use driver::lowering::Lowering;
use driver::typing::Typer;
use errors::Error;
use ir::location::SourceId;
use ir::tir;
use pp::pretty::Pretty;
use pp::theme::Theme;

pub struct Interpreter {
    all_content: String,
    content: String,
    err_code: i32,
    lowering: Lowering,
    typer: Typer,
    runtime: Runtime,
    debug_lexer: bool,
    debug_parser: bool,
    debug_typer: bool,
    theme: Theme,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut std_lib_interpreter = file_interpreter::Interpreter::stdlib();
        std_lib_interpreter.run();
        let typer = std_lib_interpreter.typer;
        let runtime = std_lib_interpreter.runtime;
        Interpreter {
            all_content: String::new(),
            content: String::new(),
            err_code: 0,
            lowering: Lowering::new(SourceId::Repl),
            typer,
            runtime,
            debug_lexer: false,
            debug_parser: false,
            debug_typer: false,
            theme: Theme::colored(),
        }
    }
}

impl Interpreter {
    pub fn add_content(&mut self, content: &str) {
        self.all_content.push_str(content);
        self.content = content.to_string();
        self.err_code = 0;
    }
}

impl interpreter::Interpreter for Interpreter {
    fn content(&self) -> &str {
        &self.content
    }

    fn source_id(&self) -> &SourceId {
        &SourceId::Repl
    }

    fn set_error_code(&mut self, code: i32) {
        self.err_code = code;
    }

    fn error_code(&self) -> i32 {
        self.err_code
    }

    fn continue_parsing(&self) -> bool {
        self.err_code == 0
    }

    fn offset_source(&self, offset: usize) -> usize {
        self.all_content.len() - self.content.len() + offset
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

    fn print<Doc>(&mut self, doc: &Doc)
    where
        Doc: Pretty,
    {
        println!("{}", doc.make_string(&self.theme));
    }

    fn print_summary(&self, def: &tir::ExpressionDefinition) {
        let summary = SummaryDefinition::from(def);
        println!("       {}", summary.make_string(&self.theme));
    }

    fn eprint(&mut self, error: &Error) {
        let mut cache = (self.source_id().clone(), Source::from(&self.all_content));
        error.eprint(self.source_id(), &self.theme, &mut cache)
    }
}
