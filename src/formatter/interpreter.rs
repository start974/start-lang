use super::parsed_file::ParsedFile;
use crate::file_interpreter::error::ErrorFileRead;
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
    content: String,
    err_code: i32,
    theme: Theme,
    file_parse: ParsedFile,
}

impl Interpreter {
    pub fn new(path: &Path) -> Self {
        let source_id = SourceId::File {
            path: path.to_path_buf(),
        };
        let mut interpreter = Interpreter {
            content: String::new(),
            err_code: 0,
            theme: Theme::default_theme(),
            file_parse: ParsedFile::new(source_id),
        };
        match read_to_string(path) {
            Ok(content) => {
                interpreter.content = content;
            }
            Err(_) => interpreter.fail(ErrorFileRead::new(path.to_path_buf())),
        };
        interpreter
    }

    /// get parsed file
    pub fn parsed_file(&self) -> Option<&ParsedFile> {
        if self.err_code == 0 {
            Some(&self.file_parse)
        } else {
            None
        }
    }

    fn source_id(&self) -> &SourceId {
        self.file_parse.source_id()
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
        let parser = parser::parse::command_offset(self.source_id().clone(), offset);
        parser.parse(content).into_result().map_err(|errs| {
            errs.iter()
                .map(|err| parser::Error::new(err.clone(), self.source_id().clone(), offset))
                .collect::<Vec<_>>()
        })
    }

    fn type_expr_definition(
        &mut self,
        _def: parser::ast::ExpressionDefinition,
    ) -> Result<typing::ast::ExpressionDefinition, Box<typing::Error>> {
        panic!("No typing for expression definitions in formatter");
    }

    fn type_ty_definition(
        &mut self,
        _def: parser::ast::TypeDefinition,
    ) -> Result<(), Box<typing::Error>> {
        panic!("No typing for type definitions in formatter");
    }

    fn type_expression(
        &mut self,
        _expr: parser::ast::Expression,
    ) -> Result<typing::ast::Expression, Box<typing::Error>> {
        panic!("No typing for type expression in formatter");
    }

    fn vm_add_definition(&mut self, _def: typing::ast::ExpressionDefinition) {
        panic!("No execution for typed definition in formatter");
    }

    fn vm_eval_expression(&mut self, _expr: typing::ast::Expression) -> vm::Value {
        panic!("No execution for expression in formatter");
    }

    fn set_debug(&mut self, _b: bool, _flag: Flag) {
        panic!("No set debug flag in formatter");
    }

    fn debug_pretty(&self, _flag: Flag, _doc: &impl Pretty) {}

    fn print_eval(&mut self, _value: &vm::Value) {}

    fn print_typeof(&mut self, _ty: &typing::ast::Type) {}

    fn eprint<E>(&self, error: &E)
    where
        E: ErrorPrint + ErrorCode,
    {
        let mut cache = (self.source_id().clone(), Source::from(&self.content));
        error.eprint(&self.theme, &mut cache).unwrap();
    }

    fn run_command(&mut self, command: parser::ast::Command) {
        self.file_parse.add_command(command);
    }
}
