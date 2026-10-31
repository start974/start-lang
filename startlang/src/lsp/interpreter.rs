use super::{backend::Backend, position_memo::PositionMemo};
use crate::interpreter::{
    self, Interpreter as _, Runtime,
    flag::{DebugFlag, Flag},
};
use driver::lowering::Lowering;
use driver::typing::Typer;
use errors::Error;
use ir::location::{SourceId, Spanned};
use ir::tir;
use pp::{MessageTheme, Pretty, Theme};
use std::path::PathBuf;
use tower_lsp::lsp_types::{Diagnostic, Url};

#[derive(Debug)]
pub struct Interpreter {
    source_id: SourceId,
    err_code: i32,
    lowering: Lowering,
    pub typer: Typer,
    pub runtime: Runtime,
    debug_lexer: bool,
    debug_parser: bool,
    debug_typer: bool,
    diagnostics: Vec<Diagnostic>,
    position_memo: PositionMemo,
}

impl Interpreter {
    pub fn new(url: &Url, content: String) -> Self {
        let mut interpreter = Interpreter::stdlib();
        interpreter.run();
        interpreter.source_id = SourceId::Url(url.to_string());
        interpreter.lowering = Lowering::new(interpreter.source_id.clone());
        interpreter.position_memo = PositionMemo::new(content);
        interpreter
    }

    /// stdlib environment
    pub fn stdlib() -> Self {
        Interpreter {
            source_id: SourceId::File(PathBuf::from("stdlib.st")),
            position_memo: PositionMemo::new(include_str!("../../../assets/stdlib.st").to_string()),
            err_code: 0,
            lowering: Lowering::new(SourceId::File(PathBuf::from("stdlib.st"))),
            typer: Typer::default(),
            runtime: Runtime::default(),
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

    ///// get document content
    //pub fn document(&mut self) -> Document {
    //use tower_lsp::lsp_types::MarkedString;

    /*        let theme = Theme::default();*/
    /*let mut document = Document::default();*/
    /*let env = self.typer.env();*/
    /*        for info in env.iter() {*/
    /*if self.source_id != info.source_id {*/
    /*continue;*/
    /*}*/
    /*document.add_symbol(SymbolInfo {*/
    /*symbol: Arc::new(info.id.as_ref().clone()),*/
    /*doc: info*/
    /*.doc*/
    /*.clone()*/
    /*.map(|doc| doc.to_string())*/
    /*.map(MarkedString::from_markdown),*/
    /*kind: info.kind,*/
    /*ty: info.ty.make_string(&theme),*/
    /*def_range: self.position_memo.range(&info.span_def),*/
    /*refs_range: info*/
    /*.span_refs*/
    /*.iter()*/
    /*.map(|span| self.position_memo.range(span))*/
    /*.collect::<Vec<_>>(),*/
    /*});*/
    /*}*/
    //document
    //}
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

    fn print<Doc>(&mut self, doc: &Doc)
    where
        Doc: Pretty + Spanned,
    {
        use tower_lsp::lsp_types::*;
        let theme = Theme::default();
        let span = doc.span();

        let range = self.position_memo.range(&span);
        let diag = Diagnostic {
            range,
            message: doc.make_string(&theme),
            related_information: None,
            severity: Some(DiagnosticSeverity::INFORMATION),
            code: None,
            code_description: None,
            source: Some(Backend::name().to_string()),
            tags: None,
            data: None,
        };
        self.diagnostics.push(diag);
    }

    fn print_summary(&self, _: &tir::ExpressionDefinition) {}

    fn eprint(&mut self, err: &Error) {
        use tower_lsp::lsp_types::*;
        let theme = MessageTheme::default();
        let span = err.span();

        let range = self.position_memo.range(&span);
        let message = err
            .text()
            .unwrap_or_else(|| err.header())
            .make_string(&theme);

        let related_information = err
            .note()
            .map(|msg| {
                let SourceId::Url(uri) = &self.source_id else {
                    unreachable!("the source of the LSP is an url")
                };
                let uri = uri.parse().expect("the source url comes from the client");
                DiagnosticRelatedInformation {
                    location: Location { uri, range },
                    message: msg.make_string(&theme),
                }
            })
            .map(|info| vec![info]);

        let diag = Diagnostic {
            range,
            message,
            related_information,
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::Number(err.code())),
            code_description: None,
            source: Some(Backend::name().to_string()),
            tags: None,
            data: None,
        };
        self.diagnostics.push(diag);
    }
}
