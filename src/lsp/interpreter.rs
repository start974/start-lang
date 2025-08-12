use super::backend::Backend;
use super::position_memo::PositionMemo;
use crate::interpreter::flag::{DebugFlag, Flag};
use crate::interpreter::{self, Interpreter as _};
use crate::typer::Typer;
use crate::utils::error::{ErrorCode, ErrorReport};
use crate::utils::location::SourceId;
use crate::utils::pretty::Pretty;
use crate::utils::theme::Theme;
use crate::vm;
use ariadne::Span as _;
use std::path::PathBuf;
use tower_lsp::lsp_types::*;

#[derive(Debug)]
pub struct Interpreter {
    source_id: SourceId,
    err_code: i32,
    pub typer: Typer,
    pub vm: vm::Env,
    debug_lexer: bool,
    debug_parser: bool,
    debug_typer: bool,
    diagnostics: Vec<Diagnostic>,
    position_memo: PositionMemo,
}

impl Interpreter {
    pub fn new(url: Url, content: String) -> Self {
        let mut interpreter = Interpreter::stdlib();
        interpreter.run();
        interpreter.source_id = SourceId::Url(url.to_string());
        interpreter.position_memo = PositionMemo::new(content);
        interpreter
    }

    /// stdlib environement
    pub fn stdlib() -> Self {
        Interpreter {
            source_id: SourceId::File(PathBuf::from("stdlib.st")),
            position_memo: PositionMemo::new(include_str!("../../assets/stdlib.st").to_string()),
            err_code: 0,
            typer: Typer::default(),
            vm: vm::Env::default(),
            debug_lexer: false,
            debug_parser: false,
            debug_typer: false,
            diagnostics: Vec::new(),
        }
    }

    /// get diagnostics
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

impl interpreter::Interpreter for Interpreter {
    fn content(&self) -> &str {
        self.position_memo.content()
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

    fn print(&mut self, _doc: &impl Pretty) {
        // TODO: push code lens
    }

    fn print_summay(&self, _: &crate::typer::ast::ExpressionDefinition) {}

    fn eprint<E>(&mut self, err: &E)
    where
        E: ErrorReport + ErrorCode,
    {
        let theme = Theme::default();
        let diag = Diagnostic {
            range: Range {
                start: self.position_memo.position(err.loc().start()),
                end: self.position_memo.position(err.loc().end()),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::Number(err.code())),
            code_description: None,
            source: Some(Backend::name().to_string()),
            message: err.message().make_string(&theme),
            related_information: None,
            tags: None,
            data: None,
        };
        self.diagnostics.push(diag);
    }
}
