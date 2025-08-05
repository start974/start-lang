use super::error::ErrorFileRead;
use crate::interpreter;
use crate::interpreter::flag::DebugFlag;
use crate::interpreter::flag::Flag;
use crate::interpreter::Interpreter as _;
use crate::parser;
use crate::typing;
use crate::utils::error::{ErrorCode, ErrorPrint};
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
    pub typer: typing::Typer,
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
            content: include_str!("../../assets/stdlib.st").to_string(),
            err_code: 0,
            typer: typing::Typer::default(),
            vm: vm::Env::default(),
            debug_lexer: false,
            debug_parser: false,
            debug_typer: false,
            theme: Theme::default_theme(),
        }
    }

    /// get flag
    fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Debug(DebugFlag::Lexer) => self.debug_lexer,
            Flag::Debug(DebugFlag::Parser) => self.debug_parser,
            Flag::Debug(DebugFlag::Typer) => self.debug_typer,
        }
    }
}

impl interpreter::Interpreter for Interpreter {
    fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn get_offset_source(&self, offset: usize) -> usize {
        offset
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

    fn type_expr_definition(
        &mut self,
        def: parser::cst::ExpressionDefinition,
        doc: Vec<String>,
    ) -> Result<typing::ast::Definition, Box<typing::Error>> {
        self.typer.definition(&def, doc)
    }

    fn type_ty_definition(
        &mut self,
        def: parser::cst::TypeDefinition,
    ) -> Result<(), Box<typing::Error>> {
        self.typer.type_definition(&def)
    }

    fn type_expression(
        &mut self,
        expr: parser::cst::Expression,
    ) -> Result<typing::ast::Expression, Box<typing::Error>> {
        self.typer.expression(&expr)
    }

    fn vm_add_definition(&mut self, def: typing::ast::Definition) {
        self.vm.add_definition(&def);
    }

    fn vm_eval_expression(&mut self, expr: typing::ast::Expression) -> vm::Value {
        self.vm.eval(&expr).unwrap()
    }

    fn set_flag(&mut self, b: bool, flag: Flag) {
        match flag {
            Flag::Debug(DebugFlag::Lexer) => self.debug_lexer = b,
            Flag::Debug(DebugFlag::Parser) => self.debug_parser = b,
            Flag::Debug(DebugFlag::Typer) => self.debug_typer = b,
        }
    }

    fn debug_pretty(&self, flag: DebugFlag, doc: &impl Pretty) {
        if self.get_flag(Flag::Debug(flag)) {
            println!("{}", doc.make_string(&self.theme));
        }
    }

    fn print_eval(&mut self, value: &vm::Value) {
        println!("{}", value.make_string(&self.theme));
    }

    fn print_typeof(&mut self, ty: &typing::ast::Type) {
        println!("{}", ty.make_string(&self.theme));
    }

    fn eprint<E>(&self, error: &E)
    where
        E: ErrorPrint + ErrorCode,
    {
        let mut cache = (self.source_id.clone(), Source::from(&self.content));
        error.eprint(&self.theme, &mut cache).unwrap();
    }
}
