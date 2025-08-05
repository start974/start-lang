use super::summary::SummaryDefinition;
use crate::file_interpreter;
use crate::interpreter;
use crate::interpreter::flag::DebugFlag;
use crate::interpreter::flag::Flag;
use crate::interpreter::Interpreter as _;
use crate::typing::ast;
use crate::typing::Typer;
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::location::SourceId;
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use crate::vm::Env;
use ariadne::Source;

pub struct Interpreter {
    all_content: String,
    content: String,
    err_code: i32,
    typer: Typer,
    vm: Env,
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
        let vm = std_lib_interpreter.vm;
        Interpreter {
            all_content: String::new(),
            content: String::new(),
            err_code: 0,
            typer,
            vm,
            debug_lexer: false,
            debug_parser: false,
            debug_typer: false,
            theme: Theme::default_theme(),
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

    fn get_error_code(&self) -> i32 {
        self.err_code
    }

    fn continue_parsing(&self) -> bool {
        self.err_code == 0
    }

    fn get_offset_source(&self, offset: usize) -> usize {
        self.all_content.len() - self.content.len() + offset
    }

    fn mut_typer(&mut self) -> &mut Typer {
        &mut self.typer
    }

    fn mut_vm(&mut self) -> &mut Env {
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

    fn print(&self, doc: &impl Pretty) {
        println!("{}", doc.make_string(&self.theme));
    }

    fn print_summay(&self, def: &ast::ExpressionDefinition) {
        let summary = SummaryDefinition::from(def);
        println!("       {}", summary.make_string(&self.theme));
    }

    fn debug(&self, flag: DebugFlag, doc: &impl Pretty) {
        if self.is_active_debug(flag) {
            self.print(doc);
        }
    }

    fn eprint<E>(&self, error: &E)
    where
        E: ErrorPrint + ErrorCode,
    {
        let mut cache = (self.source_id().clone(), Source::from(&self.all_content));
        error.eprint(&self.theme, &mut cache).unwrap();
    }
}
