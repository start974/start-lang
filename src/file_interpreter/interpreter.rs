use super::error::ErrorFileRead;
use crate::interpreter;
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
use chumsky::Parser;
use std::fs::read_to_string;
use std::path::Path;

pub struct Interpreter {
    source_id: SourceId,
    content: String,
    err_code: i32,
    pub typer: typing::Typer,
    pub vm: vm::Env,
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
            source_id: SourceId::Unknown,
            content: include_str!("../../assets/stdlib.st").to_string(),
            err_code: 0,
            typer: typing::Typer::default(),
            vm: vm::Env::default(),
            debug_parser: false,
            debug_typer: false,
            theme: Theme::default_theme(),
        }
    }

    /// get flag
    fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::DebugParser => self.debug_parser,
            Flag::DebugTyper => self.debug_typer,
        }
    }
}

impl interpreter::Interpreter for Interpreter {
    fn get_content(&self) -> &str {
        &self.content
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

    fn parse_command<'src>(
        &mut self,
        content: &'src str,
        offset: usize,
    ) -> Result<(parser::ast::Command, usize), Vec<parser::Error<'src>>> {
        let parser = parser::parse::command_offset(self.source_id.clone(), offset);
        parser.parse(content).into_result().map_err(|errs| {
            errs.iter()
                .map(|err| parser::Error::new(err.clone(), self.source_id.clone(), offset))
                .collect::<Vec<_>>()
        })
    }

    fn type_expr_definition(
        &mut self,
        def: parser::ast::ExpressionDefinition,
    ) -> Result<typing::ast::ExpressionDefinition, Box<typing::Error>> {
        self.typer.expression_definition(&def)
    }

    fn type_ty_definition(
        &mut self,
        def: parser::ast::TypeDefinition,
    ) -> Result<(), Box<typing::Error>> {
        self.typer.type_definition(&def)
    }

    fn type_expression(
        &mut self,
        expr: parser::ast::Expression,
    ) -> Result<typing::ast::Expression, Box<typing::Error>> {
        self.typer.expression(&expr)
    }

    fn vm_add_definition(&mut self, def: typing::ast::ExpressionDefinition) {
        self.vm.add_definition(&def);
    }

    fn vm_eval_expression(&mut self, expr: typing::ast::Expression) -> vm::Value {
        self.vm.eval(&expr).unwrap()
    }

    fn set_debug(&mut self, b: bool, flag: Flag) {
        match flag {
            Flag::DebugParser => self.debug_parser = b,
            Flag::DebugTyper => self.debug_typer = b,
        }
    }

    fn debug_pretty(&self, flag: Flag, doc: &impl Pretty) {
        if self.get_flag(flag) {
            println!("{}", doc.to_string(&self.theme));
        }
    }

    fn print_eval(&mut self, value: &vm::Value) {
        println!("{}", value.to_string(&self.theme));
    }

    fn print_typeof(&mut self, ty: &typing::ast::Type) {
        println!("{}", ty.to_string(&self.theme));
    }

    fn eprint<E>(&self, error: &E)
    where
        E: ErrorPrint + ErrorCode,
    {
        let mut cache = (self.source_id.clone(), Source::from(&self.content));
        error.eprint(&self.theme, &mut cache).unwrap();
    }
}
