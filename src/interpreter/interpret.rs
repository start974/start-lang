use ariadne::Span as _;

use super::error::UnknownOption;
use super::flag::Flag;
use crate::lexer;
use crate::parser::cst::AsIdentifier as _;
use crate::parser::{self, cst};
use crate::typing;
use crate::typing::ast::Typed;
use crate::utils::error::{ErrorCode, ErrorPrint};
use crate::utils::location::{Located as _, SourceId};
use crate::utils::pretty::Pretty;
use crate::vm;

pub trait Interpreter {
    /// get content
    fn content(&self) -> &str;

    /// get source id
    fn source_id(&self) -> &SourceId;

    /// set error code
    fn set_error_code(&mut self, code: i32);

    /// get error code
    fn get_error_code(&self) -> i32;

    /// continue parsing
    fn continue_parsing(&self) -> bool;

    /// get a lexer with offset
    fn get_offset_source(&self, offset: usize) -> usize;

    /// type expression defintion
    fn type_expr_definition(
        &mut self,
        def: cst::ExpressionDefinition,
    ) -> Result<typing::ast::ExpressionDefinition, Box<typing::Error>>;

    /// type type defininition
    fn type_ty_definition(&mut self, def: cst::TypeDefinition) -> Result<(), Box<typing::Error>>;

    /// type expression
    fn type_expression(
        &mut self,
        expr: cst::Expression,
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
    fn run_expr_definition(&mut self, def: cst::ExpressionDefinition) {
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
    fn run_type_definition(&mut self, def: cst::TypeDefinition) {
        if let Err(e) = self.type_ty_definition(def) {
            self.fail(e);
        }
    }

    /// run command eval
    fn run_eval(&mut self, expr: cst::Expression) {
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
    fn run_typeof(&mut self, expr: cst::Expression) {
        self.type_expression(expr)
            .map(|expr| {
                let ty = expr.ty();
                self.print_typeof(ty);
            })
            .unwrap_or_else(|e| self.fail(e))
    }

    /// run command set and unset
    fn run_set(&mut self, b: bool, var: cst::expression::Variable) {
        match var.name() {
            "DebugParser" => self.set_debug(b, Flag::DebugParser),
            "DebugTyper" => self.set_debug(b, Flag::DebugTyper),
            _ => self.fail(UnknownOption::from(var)),
        }
    }

    /// run command
    fn run_command(&mut self, cmd: cst::Command) {
        match cmd.kind {
            cst::CommandKind::ExpressionDefinition { def, .. } => self.run_expr_definition(*def),
            cst::CommandKind::TypeDefinition { def, .. } => self.run_type_definition(def),
            cst::CommandKind::Eval { expr, .. } => self.run_eval(expr),
            cst::CommandKind::TypeOf { expr, .. } => self.run_typeof(expr),
            cst::CommandKind::Set { var, .. } => self.run_set(true, var),
            cst::CommandKind::UnSet { var, .. } => self.run_set(false, var),
        }
    }

    /// lexing content
    fn lex(&mut self, content: &str, offset_source: usize) -> Vec<lexer::MetaToken> {
        let source_id = self.source_id();
        match lexer::lex(source_id.clone(), offset_source, content) {
            Ok(tokens) => tokens,
            Err(errs) => {
                self.fail(errs);
                Vec::new()
            }
        }
    }

    /// parse command with lexer tokens
    fn parse(&mut self, tokens: &[lexer::MetaToken]) -> Option<cst::Command> {
        let source_id = self.source_id();
        match parser::parse(source_id.clone(), tokens) {
            Ok(parser::CommandOrEnd::Command(cmd)) => Some(*cmd),
            Ok(parser::CommandOrEnd::End(_)) => None,
            Err(errs) => {
                self.fail(errs);
                None
            }
        }
    }

    /// run the interpreter
    fn run(&mut self) {
        if self.get_error_code() != 0 {
            return; // If there is an error code, we stop parsing
        }

        let mut offset = 0;
        let content_copy = self.content().to_string();

        while self.continue_parsing() {
            let content = &content_copy[offset..];
            if content.is_empty() {
                break;
            }
            let offset_source = self.get_offset_source(offset);
            let tokens = self.lex(content, offset_source);
            match tokens.last() {
                None => break,
                Some(last_token) => {
                    if let Some(cmd) = self.parse(&tokens) {
                        self.debug_pretty(Flag::DebugParser, &cmd);
                        self.run_command(cmd);
                    }
                    let add_offset = last_token.loc().end() - offset_source;
                    offset += add_offset;
                }
            }
        }
    }
}
