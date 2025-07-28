use super::summary::SummaryDefinition;
use crate::file_interpreter;
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

pub struct Interpreter {
    all_content: String,
    content: String,
    err_code: i32,
    typer: typing::Typer,
    vm: vm::Env,
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
        self.err_code == 0
    }

    fn parse_command<'src>(
        &mut self,
        content: &'src str,
        offset: usize,
    ) -> Result<Option<(parser::ast::Command, usize)>, Vec<parser::Error<'src>>> {
        let offset_source = self.all_content.len() - self.content.len() + offset;
        let parser = parser::parse::command_offset(SourceId::Repl, offset_source);
        parser.parse(content).into_result().map_err(|errs| {
            errs.iter()
                .map(|err| parser::Error::new(err.clone(), SourceId::Repl, offset_source))
                .collect::<Vec<_>>()
        })
    }

    fn type_expr_definition(
        &mut self,
        def: parser::ast::ExpressionDefinition,
    ) -> Result<typing::ast::ExpressionDefinition, Box<typing::Error>> {
        self.typer.expression_definition(&def).inspect(|def| {
            let summary = SummaryDefinition::from(def);
            println!("       {}", summary.to_string(&self.theme));
        })
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
        let mut cache = (SourceId::Repl, Source::from(&self.all_content));
        error.eprint(&self.theme, &mut cache).unwrap();
    }
}
