use super::error::UnknownOption;
use super::flag::Flag;
use crate::parser;
use crate::typing;
use crate::typing::ast::Typed;
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::pretty::Pretty;
use crate::vm;

pub trait Interpreter {
    /// get content
    fn get_content(&self) -> &str;

    /// set error code
    fn set_error_code(&mut self, code: i32);

    /// get error code
    fn get_error_code(&self) -> i32;

    /// continue parsing
    fn continue_parsing(&self) -> bool;

    /// parse command at offset
    fn parse_command<'src>(
        &mut self,
        content: &'src str,
        offset: usize,
    ) -> Result<(parser::ast::Command, usize), Vec<parser::Error<'src>>>;

    /// type expression defintion
    fn type_expr_definition(
        &mut self,
        def: parser::ast::ExpressionDefinition,
    ) -> Result<typing::ast::ExpressionDefinition, Box<typing::Error>>;

    /// type type defininition
    fn type_ty_definition(
        &mut self,
        def: parser::ast::TypeDefinition,
    ) -> Result<(), Box<typing::Error>>;

    /// type expression
    fn type_expression(
        &mut self,
        expr: parser::ast::Expression,
    ) -> Result<typing::ast::Expression, Box<typing::Error>>;

    /// add definitin in vm
    fn vm_add_definition(&mut self, def: typing::ast::ExpressionDefinition);

    /// eval expression in vm
    fn vm_eval_expression(&mut self, expr: typing::ast::Expression) -> vm::Value;

    /// set debug parser
    fn set_debug(&mut self, b: bool, flag: Flag);

    /// pretty debug
    fn debug_pretty(&self, flag: Flag, doc: &impl Pretty);

    /// print eval
    fn print_eval(&mut self, value: &vm::Value);

    /// print type of
    fn print_typeof(&mut self, ty: &typing::ast::Type);

    /// print error
    fn eprint<E>(&self, error: &E)
    where
        E: ErrorPrint + ErrorCode;

    /// fail with error
    fn fail<E>(&mut self, error: E)
    where
        E: ErrorPrint + ErrorCode,
    {
        self.eprint(&error);
        let code = if self.get_error_code() == 0 {
            error.code()
        } else {
            1
        };
        self.set_error_code(code);
    }

    /// run command expr definition
    fn run_expr_definition(&mut self, def: parser::ast::ExpressionDefinition) {
        self.type_expr_definition(def)
            .map(|def| {
                self.debug_pretty(Flag::DebugTyper, &def);
                if self.get_error_code() == 0 {
                    self.vm_add_definition(def)
                }
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    /// run command type definition
    fn run_type_definition(&mut self, def: parser::ast::TypeDefinition) {
        if let Err(e) = self.type_ty_definition(def) {
            self.fail(e);
        }
    }

    /// run command eval
    fn run_eval(&mut self, expr: parser::ast::Expression) {
        self.type_expression(expr)
            .map(|expr| {
                self.debug_pretty(Flag::DebugTyper, &expr);
                if self.get_error_code() == 0 {
                    let value = self.vm_eval_expression(expr);
                    self.print_eval(&value);
                }
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    /// run type of expression
    fn run_typeof(&mut self, expr: parser::ast::Expression) {
        self.type_expression(expr)
            .map(|expr| {
                let ty = expr.ty();
                self.print_typeof(ty);
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    /// run command set and unset
    fn run_set(&mut self, b: bool, identifier: parser::ast::Identifier) {
        match identifier.name() {
            "DebugParser" => self.set_debug(b, Flag::DebugParser),
            "DebugTyper" => self.set_debug(b, Flag::DebugTyper),
            _ => self.fail(UnknownOption::from(identifier)),
        }
    }

    /// run command
    fn run_command(&mut self, cmd: parser::ast::Command) {
        match cmd {
            parser::ast::Command::ExpressionDefinition(def) => self.run_expr_definition(def),
            parser::ast::Command::TypeDefinition(def) => self.run_type_definition(def),
            parser::ast::Command::Eval(expr) => self.run_eval(expr),
            parser::ast::Command::TypeOf(expr) => self.run_typeof(expr),
            parser::ast::Command::Set(b, id) => self.run_set(b, id),
        }
    }

    /// run the interpreter
    fn run(&mut self) {
        let mut offset = 0;
        let mut content = self.get_content().to_string();

        while !content.is_empty() && self.continue_parsing() {
            match self.parse_command(&content, offset) {
                Ok((cmd, add_offset)) => {
                    offset += add_offset;
                    content = content[add_offset..].to_string();
                    self.debug_pretty(Flag::DebugParser, &cmd);
                    self.run_command(cmd);
                }
                Err(errs) => {
                    self.fail(errs);
                    break;
                }
            }
        }
    }
}
